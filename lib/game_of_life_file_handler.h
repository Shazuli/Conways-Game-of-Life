#ifndef __GAME_OF_LIFE_FILE_HANDLER_H__
#define __GAME_OF_LIFE_FILE_HANDLER_H__

#include <stdio.h>
#include "game_of_life_field.h"

char load_config_from_stream(Field *field, FILE *fp);

char save_config_to_stream(const Field field, FILE *fp);

#endif