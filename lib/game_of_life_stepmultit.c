#include "util.h"
#include "game_of_life_field.h"

#include "type_defs.h"

#include <stdlib.h>
#include <pthread.h>

struct Field{
    u16 rows;
    u16 columns;
    u16 blocks;
    volatile t_block_field **current;
    t_block_field **next;
};


struct FieldData{
    Field *field;
    u16 row;
    u16 block;
};

static void* calcByte(void* arg)
{
    struct FieldData *fieldData = (struct FieldData*)arg;

    Field *field = fieldData->field;

    const u16 r = fieldData->row;
    const u16 b = fieldData->block;
    u16 c;

    // Data copied, free up that memory.
    free(fieldData);

    // Game logic.
    i8 alive, cnt;
    for (u8 bo=0;bo<8;bo++) {
        c = b * 8 + bo;

        // Prevent out of bounds.
        if (c >= field->columns) {
            return NULL;
        }

        alive = fieldIsAlive(field,r,c);
        cnt = -alive;

        // Count 9x9
        if (r == 0) {
            for (u16 ro=r;ro<MIN(r+2,field->rows);ro++) {
                if (c == 0) {
                    for (u16 co=c;co<MIN(c+2,field->columns);co++) {
                        cnt += fieldIsAlive(field,ro,co);
                    }
                } else {
                    for (u16 co=c-1;co<MIN(c+2,field->columns);co++) {
                        cnt += fieldIsAlive(field,ro,co);
                    }
                }
            }
        } else {
            for (u16 ro=r-1;ro<MIN(r+2,field->rows);ro++) {
                if (c == 0) {
                    for (u16 co=c;co<MIN(c+2,field->columns);co++) {
                        cnt += fieldIsAlive(field,ro,co);
                    }
                } else {
                    for (u16 co=c-1;co<MIN(c+2,field->columns);co++) {
                        cnt += fieldIsAlive(field,ro,co);
                    }
                }
            }
        }
        
        
        if (alive) {
            if (cnt == 2 || cnt == 3) {// Living cell has 3 or 4 living neighbours survives
                SET_BIT(field->next[r][c/8], c % 8);
            } else {
                CLEAR_BIT(field->next[r][c/8], c % 8);
            }
        } else {
            if (cnt == 3) {// Dead cell becomes alive if it has exactly 3 living neighbours
                SET_BIT(field->next[r][c/8], c % 8);
            } else {
                CLEAR_BIT(field->next[r][c/8], c % 8);
            }
        }
    }

    return NULL;
}


void fieldStepMultit(Field *field)
{
    pthread_t *pthreadIDs = malloc(field->rows * field->blocks * sizeof(pthread_t));

    
    for (u16 r=0;r<field->rows;r++) {
        for (u16 b=0;b<field->blocks;b++) {
            
            // Store data for the thread.
            struct FieldData *fieldData = malloc(sizeof(struct FieldData));
            fieldData->field = field;
            fieldData->row = r;
            fieldData->block = b;

            // Create 1 thread for each byte.
            pthread_create(&pthreadIDs[r * field->blocks + b], NULL, &calcByte, fieldData);
        }
    }

    // Wait for them all to finish.
    for (u16 i=0;i<field->rows * field->blocks;i++) {
        pthread_join(pthreadIDs[i], NULL);
    }

    free(pthreadIDs);
}