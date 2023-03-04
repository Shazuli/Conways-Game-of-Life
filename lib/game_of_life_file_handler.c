#include "game_of_life_field.h"
#include "game_of_life_file_handler.h"

#include <stdio.h>
#include <stdlib.h>

char load_config_from_stream(Field *field, FILE *stream)
{
    int rows, columns;
    fscanf(stream,"%dx%d\n",&rows,&columns);
    *field = field_new(rows,columns);

    for (unsigned short r=0;r<field->rows;r++) {
        for (unsigned short b=0;b<field->blocks;b++) {
            unsigned int byte;
            fscanf(stream,"%02x",&byte);
            field->current[r][b] = byte;
        }
        fscanf(stream,"\n");
    }

    return 0;
}

char save_config_to_stream(const Field field, FILE *stream)
{
    if (stream == NULL) {
        return 1;
    }
    fprintf(stream,"%dx%d\n",field.rows,field.columns);
    for (unsigned short r=0;r<field.rows;r++) {
        for (unsigned short b=0;b<field.blocks;b++) {
            fprintf(stream,"%02x",field.current[r][b]);
        }
        fprintf(stream,"\n");
    }
    return 0;
}