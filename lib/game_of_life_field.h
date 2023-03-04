#ifndef __GAME_OF_LIFE_FIELD_H__
#define __GAME_OF_LIFE_FIELD_H__

// The type for storing a byte.
#define t_block_field unsigned char

typedef struct {
    unsigned short rows;
    unsigned short columns;
    unsigned short blocks;
    t_block_field **current;
    t_block_field **next;
} Field;


char field_isAlive(Field *field, unsigned short row, unsigned short column);
void field_setAlive(Field *field, unsigned short row, unsigned short column);
void field_setDead(Field *field, unsigned short row, unsigned short column);
void field_setAllDead(Field *field);
void field_moveCurrentGenerationToNext(Field *field);
void field_stepSinglet(Field *field);
//void step_multit(Field *field, int n);


Field field_new(unsigned short rows, unsigned short columns);
void field_free(Field *field);

#endif