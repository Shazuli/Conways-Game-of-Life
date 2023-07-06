#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include <string.h>

#include "../lib/game_of_life_field.h"
#include "../lib/game_of_life_file_handler.h"

// Symbols for cell states.
#define ALIVE 'X'
#define DEAD '.'
#define SPACE "              "

char DEBUG = 0;

/*static char loadFieldFromFile(char *fileName, Field *field)
{
    FILE *in_file_p;
    if (!(in_file_p = fopen(fileName,"r"))) {// Input file
        fprintf(stderr,"Could not open the file: '%s'\n",fileName);
        return 0;
    }
    load_config_from_stream(field,in_file_p);
    fclose(in_file_p);
    return 1;
}*/

/*static void printSize(Field *field)
{
    unsigned short sum = 0;
    for (unsigned short r=0;r<field->rows;r++) {
        for (unsigned short b=0;b<field->blocks;b++) {
            sum += sizeof(field->current[r][b]) + sizeof(field->next[r][b]);
        }
    }
    printf("%ix%i (%u Bytes)\n",field->rows,field->columns,sum);
}*/

static void clearBuffer()
{
    int ch;
    while ((ch = getchar()) != '\n' && ch != EOF) {}
}

void drawField(Field *field);
void setRandomSeed(Field *field);
static Field *initField(unsigned short rows, unsigned short columns);

int main(int argc, char * const argsv[])
{
    char is_loaded_from_file = 0;
    unsigned short rows = 20, columns = 20;// Default size
    Field *fieldMatrix;
    int option;

    // Argument flags.
    while ((option = getopt(argc,argsv,"sf")) != -1) {
        switch (option) {
            case 's':
                if (argc < 4) {
                    printf("Usage %s -s <rows> <columns>\n",argsv[0]);
                    return -1;
                }
                rows = atoi(argsv[2]);
                columns = atoi(argsv[3]);

                if (rows == 0 || columns == 0) {
                    puts("Size can't be 0");
                    return -1;
                }
                continue;
            case 'f':
                if (argc < 3) {
                    printf("Usage %s -f <fileName>\n",argsv[0]);
                    return -1;
                }
                FILE *in_file_p;
                if (!(in_file_p = fopen(argsv[2],"r"))) {// Input file
                    fprintf(stderr,"Could not open the file: '%s'\n",argsv[2]);
                    return 0;
                }
                //fieldMatrix = load_config_from_stream(in_file_p);
                fieldMatrix = fieldDeserialize(in_file_p);
                rows = fieldGetRows(fieldMatrix);
                columns = fieldGetColumns(fieldMatrix);
                fclose(in_file_p);
                is_loaded_from_file = 1;
                continue;
        }
    }

    if (!is_loaded_from_file) {
        fieldMatrix = initField(rows, columns);
    }

    char ch, speed = 1;

    while (1)
    {
        drawField(fieldMatrix);

        puts("Select one of the following options:");
        puts("(enter) Step\n(1-9)   Speed\n(any)   Exit");

        ch = getchar();

        if (ch > '0' && ch <= '9') {// Set speed
            speed = ch-'0';
            printf("\033[%iA",5+columns);// Go back to top (if possible)
            clearBuffer();
        } else if (ch == 's' || ch == 'S') {// Save to file
            char fileName[16];
            printf("Name of save:\n");
            scanf("%15s",fileName);
            FILE *out_file_p;
            //if (!(out_file_p = fopen(strcat(fileName, ".txt"),"w"))) {
            if (!(out_file_p = fopen(fileName,"w"))) {
                fprintf(stderr,"Could not open file: %s",fileName);
            } else {
                //save_config_to_stream(fieldMatrix,out_file_p);
                fieldSerialize(fieldMatrix,out_file_p);
            }
            fclose(out_file_p);
            //fieldSerialize(fieldMatrix, stdout);
            clearBuffer();
		} else if (ch != '\n') {// Quit
			break;
		} else {// Step simulation
            /*clock_t start, end;
            start = clock();
            
            end = clock();
            

            printf("Took %lf\n",(double)end-start);*/
            for (char i=0;i<speed;i++) {
                fieldStepSinglet(fieldMatrix);
                fieldMoveCurrentGenerationToNext(fieldMatrix);
            }
            printf("\033[%iA",5+columns);// Go back to top (if possible)
        }
    }
    fieldFree(fieldMatrix);
    return 0;
}

/*
 * Description: Draw the field.
 * Input:       Field struct.
 */
void drawField(Field *field)
{
    for (unsigned short c=0;c<fieldGetColumns(field);c++) {
        //for (short r=field->rows;r>=0;r--) {// Bit order
        for (unsigned short r=0;r<fieldGetRows(field);r++) {// Display order
            if (fieldIsAlive(field,r,c)) {
                printf("%c ", ALIVE);
            } else {
                printf("%c ", DEAD);
            }
        }
        /*if (DEBUG > 1) {
            for (short b=field->blocks-1;b>=0;b--) {
                printf("%lx",(unsigned long) field->current[c][b]);
            }
            printf("%s\n",SPACE);
            //printf("%lx%s\n",(unsigned long)field->current[c][0],SPACE);

        } else if (DEBUG > 0) {
            for (short b=field->blocks-1;b>=0;b--) {
                printf("%lu",(unsigned long) field->current[c][b]);
            }
            printf("%s\n",SPACE);
            //printf("%lu%s\n",(unsigned long)field->current[c][0],SPACE);
        } else {
            printf("\n");
        }*/
        printf("\n");
    }
}

/*
 * Description: Generate field from random seed.
 * Input        Field struct.
 */
void setRandomSeed(Field *field)
{
    for (unsigned short r=0;r<fieldGetRows(field);r++) {
        for (unsigned short b=0;b<fieldGetBlocks(field);b++) {
            //field->current[r][b] = rand();
            *fieldGetAt(field, r, b) = rand();
        }
    }
}

/* Description: Initialize all the cells to dead, then asks the user
 *              about which structure to load, and finally load the
 *              structure.
 * Input:       Field struct, rows, columns.
 */
static Field *initField(unsigned short rows, unsigned short columns)
{
    printf("Select field spec to load ");
	printf("([G]lider, [S]emaphore, ");
    printf("[S]ave game, ");
    printf("[P]redetermined random, ");
	printf("[R]andom or [C]ustom):\n");

    Field *field;

    // Row order is mirrored.
    switch (getchar()) {
        case 'g':
        case 'G':
            // Create glider pattern.
            field = fieldNew(rows,columns);
            *fieldGetAt(field, 0, 0) = 2;
            *fieldGetAt(field, 1, 0) = 4;
            *fieldGetAt(field, 2, 0) = 7;
            break;
        case 's':
        case 'S':
            // Create semaphore pattern.
            field = fieldNew(rows,columns);
            *fieldGetAt(field, 3, 0) = 14;
            break;
        case 'p':
        case 'P':
            {
                // Create random field from seed.
                field = fieldNew(rows,columns);
                int seed;
                puts("Input seed:");
                scanf("%i",&seed);
                srand(seed);
                setRandomSeed(field);
            }
            break;
        case 'r':
        case 'R':
            // Set random field from current time.
            field = fieldNew(rows,columns);
            srand(time(0));
            setRandomSeed(field);
            break;
        default:
            {
                puts("Nope.");
            }

    }
    clearBuffer();
    return field;
}