#include "game_of_life_field.h"
#include "game_of_life_file_handler.h"

#include "type_defs.h"

#include <stdio.h>
#include <stdlib.h>

struct Field{
    u16 rows;
    u16 columns;
    u16 blocks;
    t_block_field **current;
    t_block_field **next;
};

//#define TO_BE_BYTES(var) ({u8 buf[sizeof(var)]; buf;})

Field *load_config_from_stream(FILE *stream)
{
    u16 rows, columns;
    fscanf(stream,"%hux%hu\n",&rows,&columns);
    Field *field = fieldNew(rows,columns);

    for (u16 r=0;r<field->rows;r++) {
        for (u16 b=0;b<field->blocks;b++) {
            unsigned int byte;
            fscanf(stream,"%02x",&byte);
            field->current[r][b] = byte;
        }
        fscanf(stream,"\n");
    }

    return field;
}

i8 save_config_to_stream(Field *field, FILE *stream)
{
    if (stream == NULL) {
        return 1;
    }
    fprintf(stream,"%hux%hu\n",field->rows,field->columns);
    for (u16 r=0;r<field->rows;r++) {
        for (u16 b=0;b<field->blocks;b++) {
            fprintf(stream,"%02x",field->current[r][b]);
        }
        fprintf(stream,"\n");
    }
    return 0;
}

i8 fieldSerialize(Field *field, FILE *stream)
{
    if (stream == NULL) {
        return 1;
    }

    u16 rows = field->rows >> 8 | field->rows << 8;// In BE order

    fwrite(&rows, 1, sizeof(field->rows), stream);



    u8 data = field->columns % 8;
    fwrite(&data, 1, sizeof(data), stream);

    for (u16 r=0;r<field->rows;r++) {
        fwrite(field->current[r], 1, field->blocks, stream);
    }

    return 0;
}

Field *fieldDeserialize(FILE *stream)
{
    u16 rows;
    {
        u8 buf[2];
        fread(buf, sizeof(u16), 1, stream);
        rows = buf[0] << 8 | buf[1];// To BE order
    }

    u8 data;
    fread(&data, 1, sizeof(u8), stream);
    
    u8 offset = data & 7;// First 3 bits are the offset


    u16 length = 0;
    u8 *buf = calloc(rows * rows, sizeof(u8));// Hacky, should be LinkedList

    while (fread(&buf[length++], 1, sizeof(u8), stream) != 0) {}

    u16 columns = (length / rows - (offset > 0)) * 8 + offset;

    Field *field = fieldNew(rows, columns);

    for (u16 r=0;r<field->rows;r++) {
        for (u16 b=0;b<field->blocks;b++) {
            field->current[r][b] = buf[r * field->blocks + b];
        }
    }

    free(buf);

    return field;
}