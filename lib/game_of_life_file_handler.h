#ifndef __GAME_OF_LIFE_FILE_HANDLER_H__
#define __GAME_OF_LIFE_FILE_HANDLER_H__

#include "type_defs.h"

#include <stdio.h>
#include "game_of_life_field.h"

Field *load_config_from_stream(FILE *stream);

i8 save_config_to_stream(Field *field, FILE *fp);

/**
 * @brief Write Field struct to stream.
 * @param[in] field Field struct.
 * @param[in] stream Stream.
 */
i8 fieldSerialize(Field *field, FILE *stream);
/**
 * @brief Read a stream to a Field struct.
 * @param[in] stream Stream.
 * @return Field struct.
 */
Field *fieldDeserialize(FILE *stream);

#endif