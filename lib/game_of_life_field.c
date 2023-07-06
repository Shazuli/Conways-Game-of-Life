#include "util.h"
#include "game_of_life_field.h"

#include "type_defs.h"

#include <stdlib.h>

struct Field{
    u16 rows;
    u16 columns;
    u16 blocks;
    t_block_field **current;
    t_block_field **next;
};

/*
 * Description: Create a field struct with everyone dead.
 * Input:       How many rows, how many columns.
 * Output:      Pointer to Field struct.
 */
Field *fieldNew(u16 rows, u16 columns)
{
    u16 blockSize = columns / 8 + (columns % 8 > 0);

    /*Field field = {
        .rows = rows,
        .columns = columns,
        .blocks = blockSize,
        .current = malloc(rows * sizeof(t_block_field *)),
        .next = malloc(rows * sizeof(t_block_field *))
    };*/

    Field *field = malloc(sizeof(Field));
    field->rows = rows;
    field->columns = columns;
    field->blocks = blockSize;
    field->current = malloc(rows * sizeof(t_block_field *));
    field->next = malloc(rows * sizeof(t_block_field *));

    for (u16 r=0;r<rows;r++) {
        field->current[r] = (t_block_field *) calloc(blockSize,sizeof(t_block_field));
        field->next[r] = (t_block_field *) calloc(blockSize,sizeof(t_block_field));
    }

    return field;
}


u16 fieldGetRows(Field *field)
{
    return field->rows;
}

u16 fieldGetColumns(Field *field)
{
    return field->columns;
}

u16 fieldGetBlocks(Field *field)
{
    return field->blocks;
}

t_block_field *fieldGetAt(Field *field, u16 row, u16 block)
{
    return &field->current[row][block];
}

void fieldFree(Field *field)
{
    for (u16 r=0;r<field->rows;r++) {
        free(field->current[r]);
        free(field->next[r]);
    }
    free(field->current);
    free(field->next);
    free(field);
}

/*
 * Description: Step the simulation once in a single thread.
 * Input:       Field struct.
 */
void fieldStepSinglet(Field *field)
{
    // Game logic.
    i8 alive, cnt;
    for (u16 r=0;r<field->rows;r++) {
        for (u16 c=0;c<field->columns;c++) {
            alive = fieldIsAlive(field,r,c);
            cnt = -alive;

            /*for (short ro=r-1;ro<(r+2);ro++) {
                for (short co=c-1;co<(c+2);co++) {
                    cnt += isAlive(field,ro,co);
                }
            }*/
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
    }
}

/*
 * Description: Move current generation to next generation.
 * Input:       Field struct.
 */
void fieldMoveCurrentGenerationToNext(Field *field)
{
    for (short r=0;r<field->rows;r++) {
        for (u16 b=0;b<field->blocks;b++) {
            field->current[r][b] = field->next[r][b];
        }
    }
}

/*
 * Description: Get if cell at position is alive.
 * Input:       Field struct, row, column.
 * Output:      1 if alive, 0 if dead.
 */
i8 fieldIsAlive(Field *field, u16 row, u16 column)
{
    if (column >= field->columns ||
        row >= field->rows) {
            return 0;
    }
    return (field->current[row][column / 8] & 1<<((column % 8))) >= 1;
}

/*
 * Description: Set cell at position to alive.
 * Input:       Field struct, row, column.
 */
i8 fieldSetAlive(Field *field, u16 row, u16 column)
{
    if (column >= field->columns ||
        row >= field->rows) {
            return 0;
    }
    SET_BIT(field->next[row][column / 8], column % 8);
    return 1;
}

/*
 * Description: Set cell at position to dead.
 * Input:       Field struct, row, column.
 */
i8 fieldSetDead(Field *field, u16 row, u16 column)
{
    if (column >= field->columns ||
        row >= field->rows) {
            return 0;
    }
    CLEAR_BIT(field->next[row][column / 8], column % 8);
    return 1;
}

/*
 * Description: Set all cells to dead.
 * Input:       Field struct.
 */
void fieldSetAllDead(Field *field)
{
    for (short r=0;r<field->rows;r++) {
        for (u16 b=0;b<field->blocks;b++) {
            field->next[r][b] ^= field->next[r][b];
        }
    }
}