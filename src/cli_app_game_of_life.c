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

static char loadFieldFromFile(char *fileName, Field *field)
{
    FILE *in_file_p;
    if (!(in_file_p = fopen(fileName,"r"))) {// Input file
        fprintf(stderr,"Could not open the file: '%s'\n",fileName);
        return 1;
    }
    load_config_from_stream(field,in_file_p);
    fclose(in_file_p);
    return 0;
}

static void printSize(Field *field)
{
    unsigned short sum = 0;
    for (unsigned short r=0;r<field->rows;r++) {
        for (unsigned short b=0;b<field->blocks;b++) {
            sum += sizeof(field->current[r][b]) + sizeof(field->next[r][b]);
        }
    }
    printf("%ix%i (%u Bytes)\n",field->rows,field->columns,sum);
}

static void clearBuffer()
{
    int ch;
    while ((ch = getchar()) != '\n' && ch != EOF) {}
}

void drawField(Field *field);
void setRandomSeed(Field *field);
static void initField(Field *field, unsigned short rows, unsigned short columns);

int main(int argc, char * const argsv[])
{
    char is_loaded_from_file = 0;
    unsigned short rows = 20, columns = 20;// Default size
    Field fieldMatrix;
    int option;

    // Argument flags.
    while ((option = getopt(argc,argsv,"dsf")) != -1) {
        switch (option) {
            case 'd':
                DEBUG++;
                continue;
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
                loadFieldFromFile(argsv[2],&fieldMatrix);
                is_loaded_from_file = 1;
                continue;
        }
    }

    //Field fieldMatrix = field_new(rows,columns);

    if (!is_loaded_from_file) {
        initField(&fieldMatrix,rows,columns);
    }

    printSize(&fieldMatrix);

    char ch, speed = 1;

    while (1)
    {
        drawField(&fieldMatrix);

        puts("Select one of the following options:");
        puts("(enter) Step\n(1-9)   Speed\n(any)   Exit");

        ch = getchar();

        if (ch > '0' && ch <= '9') {// Set speed
            speed = ch-'0';
            printf("\033[%iA",5+fieldMatrix.rows);// Go back to top (if possible)
            clearBuffer();
        } else if (ch == 's' || ch == 'S') {// Save to file
            char fileName[16];
            printf("Name of save:\n");
            scanf("%15s",fileName);
            FILE *out_file_p;
            if (!(out_file_p = fopen(strcat(fileName, ".txt"),"w"))) {
                fprintf(stderr,"Could not open file: %s",fileName);
            } else {
                save_config_to_stream(fieldMatrix,out_file_p);
            }
            fclose(out_file_p);
            clearBuffer();
		} else if (ch != '\n') {// Quit
            field_free(&fieldMatrix);
			break;
		} else {// Step simulation
            /*clock_t start, end;
            start = clock();
            
            end = clock();
            

            printf("Took %lf\n",(double)end-start);*/
            for (char i=0;i<speed;i++) {
                field_stepSinglet(&fieldMatrix);
                field_moveCurrentGenerationToNext(&fieldMatrix);
            }
            printf("\033[%iA",5+fieldMatrix.columns);// Go back to top (if possible)
        }
    }
    return 0;
}

/*
 * Description: Draw the field.
 * Input:       Field struct.
 */
void drawField(Field *field)
{
    for (unsigned short c=0;c<field->columns;c++) {
        //for (short r=field->rows;r>=0;r--) {// Bit order
        for (unsigned short r=0;r<field->rows;r++) {// Display order
            if (field_isAlive(field,r,c)) {
                printf("%c ",ALIVE);
            } else {
                printf("%c ",DEAD);
            }
        }
        if (DEBUG > 1) {
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
        }
    }
}

/*
 * Description: Generate field from random seed.
 * Input        Field struct.
 */
void setRandomSeed(Field *field)
{
    for (unsigned short r=0;r<field->rows;r++) {
        for (unsigned short b=0;b<field->blocks;b++) {
            field->current[r][b] = rand();
        }
    }
}

/* Description: Initialize all the cells to dead, then asks the user
 *              about which structure to load, and finally load the
 *              structure.
 * Input:       Field struct, rows, columns.
 */
static void initField(Field *field, unsigned short rows, unsigned short columns)
{
    printf("Select field spec to load ");
	printf("([G]lider, [S]emaphore, ");
    printf("[S]ave game, ");
    printf("[P]redetermined random, ");
	printf("[R]andom or [C]ustom):\n");

    // Row order is mirrored.
    switch (getchar()) {
        case 'g':
        case 'G':
            // Create glider structure.
            *field = field_new(rows,columns);
            field->current[0][0] = 2;
            field->current[1][0] = 4;
            field->current[2][0] = 7;
            break;
        case 's':
        case 'S':
            // Create semaphore structure.
            *field = field_new(rows,columns);
            field->current[3][0] = 14;
            break;
        case 'p':
        case 'P':
            {
                // Create random field from seed.
                *field = field_new(rows,columns);
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
            *field = field_new(rows,columns);
            srand(time(0));
            setRandomSeed(field);
            break;
        default:
            {
                puts("Nope.");
            }

    }
    clearBuffer();
}