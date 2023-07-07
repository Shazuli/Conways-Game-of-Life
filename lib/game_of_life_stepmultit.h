#ifndef __GAME_OF_LIFE_STEPMULTIT_H__
#define __GAME_OF_LIFE_STEPMULTIT_H__

#include "type_defs.h"
#include "game_of_life_field.h"

/**
 * @brief Step the simulation once in multiple threads and stores the result in memory.
 * @param[in] field Field struct.
 */
void fieldStepMultit(Field *field);

#endif