#include "util.h"
#include "game_of_life_field.h"

#include <stdlib.h>

static unsigned short totalBlocks(unsigned short columns)
{
    return columns / 8 + (columns % 8 > 0);
}

/*
 * Description: Create a field struct with everyone dead.
 * Input:       How many rows, how many columns.
 * Output:      Field struct.
 */
Field field_new(unsigned short rows, unsigned short columns)
{
    unsigned short blockSize = totalBlocks(columns);

    Field field = {
        .rows = rows,
        .columns = columns,
        .blocks = blockSize,
        .current = malloc(rows * sizeof(t_block_field *)),
        .next = malloc(rows * sizeof(t_block_field *))
    };
    for (unsigned short r=0;r<rows;r++) {
        field.current[r] = (t_block_field *) calloc(blockSize,sizeof(t_block_field));
        field.next[r] = (t_block_field *) calloc(blockSize,sizeof(t_block_field));
    }

    return field;
}


void field_free(Field *field)
{
    for (unsigned short r=0;r<field->rows;r++) {
        free(field->current[r]);
        free(field->next[r]);
    }
    free(field->current);
    free(field->next);
}

/*
 * Description: Step the simulation once in a single thread.
 * Input:       Field struct.
 */
void field_stepSinglet(Field *field)
{
    // Game logic.
    char alive, cnt;
    for (unsigned short r=0;r<field->rows;r++) {
        for (unsigned short c=0;c<field->columns;c++) {
            alive = field_isAlive(field,r,c);
            cnt = -alive;

            /*for (short ro=r-1;ro<(r+2);ro++) {
                for (short co=c-1;co<(c+2);co++) {
                    cnt += isAlive(field,ro,co);
                }
            }*/
            // Count 9x9
            if (r == 0) {
                for (unsigned short ro=r;ro<MIN(r+2,field->rows);ro++) {
                    if (c == 0) {
                        for (unsigned short co=c;co<MIN(c+2,field->columns);co++) {
                            cnt += field_isAlive(field,ro,co);
                        }
                    } else {
                        for (unsigned short co=c-1;co<MIN(c+2,field->columns);co++) {
                            cnt += field_isAlive(field,ro,co);
                        }
                    }
                }
            } else {
                for (unsigned short ro=r-1;ro<MIN(r+2,field->rows);ro++) {
                    if (c == 0) {
                        for (unsigned short co=c;co<MIN(c+2,field->columns);co++) {
                            cnt += field_isAlive(field,ro,co);
                        }
                    } else {
                        for (unsigned short co=c-1;co<MIN(c+2,field->columns);co++) {
                            cnt += field_isAlive(field,ro,co);
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
void field_moveCurrentGenerationToNext(Field *field)
{
    for (short r=0;r<field->rows;r++) {
        for (unsigned short b=0;b<field->blocks;b++) {
            field->current[r][b] = field->next[r][b];
        }
    }
}

/*
 * Description: Get if cell at position is alive.
 * Input:       Field struct, row, column.
 * Output:      1 if alive, 0 if dead.
 */
char field_isAlive(Field *field, unsigned short row, unsigned short column)
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
void field_setAlive(Field *field, unsigned short row, unsigned short column)
{
    if (column >= field->columns ||
        row >= field->rows) {
            return;
    }
    SET_BIT(field->next[row][column / 8], column % 8);
}

/*
 * Description: Set cell at position to dead.
 * Input:       Field struct, row, column.
 */
void field_setDead(Field *field, unsigned short row, unsigned short column)
{
    if (column >= field->columns ||
        row >= field->rows) {
            return;
    }
    CLEAR_BIT(field->next[row][column / 8], column % 8);
}

/*
 * Description: Set all cells to dead.
 * Input:       Field struct.
 */
void field_setAllDead(Field *field)
{
    for (short r=0;r<field->rows;r++) {
        for (unsigned short b=0;b<field->blocks;b++) {
            field->next[r][b] ^= field->next[r][b];
        }
    }
}