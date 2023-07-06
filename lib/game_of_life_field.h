#ifndef __GAME_OF_LIFE_FIELD_H__
#define __GAME_OF_LIFE_FIELD_H__

#include "type_defs.h"

// The type for storing a byte.
#define t_block_field u8

typedef struct Field Field;

/**
 * @brief Get rows of Field struct.
 * @param[in] field Field struct.
 */
u16 fieldGetRows(Field *field);
/**
 * @brief Get columns of Field struct.
 * @param[in] field Field struct.
 */
u16 fieldGetColumns(Field *field);
/**
 * @brief Get blocks of Field struct.
 * @param[in] field Field struct.
 */
u16 fieldGetBlocks(Field *field);
/**
 * @brief Get pointer to block in Field struct.
 * @param[in] field Field struct.
 * @param[in] row Row.
 * @param[in] block Block.
 * @return Pointer to block.
 */
t_block_field *fieldGetAt(Field *field, u16 row, u16 block);
/**
 * @brief Get if cell at position is alive.
 * @param[in] field Field struct.
 * @param[in] row Row.
 * @param[in] column Column.
 * @return True if alive, false otherwise.
 */
i8 fieldIsAlive(Field *field, u16 row, u16 column);
/**
 * @brief Set cell at position to alive.
 * @param[in] field Field struct.
 * @param[in] row Row.
 * @param[in] column Column.
 * @return True on success, false otherwise.
 */
i8 fieldSetAlive(Field *field, u16 row, u16 column);
/**
 * @brief Set cell at position to dead.
 * @param[in] field Field struct.
 * @param[in] row Row.
 * @param[in] column Column.
 * @return True on success, false otherwise.
 */
i8 fieldSetDead(Field *field, u16 row, u16 column);
/**
 * @brief Sets all cells in Field struct to dead.
 * @param[in] field Field struct.
 */
void fieldSetAllDead(Field *field);
/**
 * @brief Make next generation current generation in Field struct. 
 * @param[in] field Field struct.
 */
void fieldMoveCurrentGenerationToNext(Field *field);
/**
 * @brief Step the simulation once in a single thread and stores the result in memory.
 * @param[in] field Field struct.
 */
void fieldStepSinglet(Field *field);
//void stepMultit(Field *field, int n);

/**
 * @brief Creates a new Field struct.
 * @param[in] rows Rows.
 * @param[in] columns Columns.
 * @return Pointer to new Field struct.
 */
Field *fieldNew(u16 rows, u16 columns);
/**
 * @brief Free memory of Field struct.
 * @param[in] field Field struct.
 */
void fieldFree(Field *field);

#endif