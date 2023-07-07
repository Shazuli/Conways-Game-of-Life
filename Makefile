tarFile := src_$(shell date +%Y-%m-%d-%H%M).tar.gz

.PHONY: cli_app_game_of_life clean $(tarFile)

#riscv64-gcc
CC=gcc
FLAGS = -g -Wall

cli_app_game_of_life: src/cli_app_game_of_life.c build/game_of_life_field.o build/game_of_life_stepmultit.o build/game_of_life_file_handler.o
	$(CC) $(FLAGS) -o $@ $^

build/%.o: lib/%.c lib/%.h lib/util.h | build
	$(CC) $(FLAGS) -c -o $@ $<

build:
	-mkdir $@

$(tarFile):
	tar czvf $@ src lib Makefile

clean:
	rm -r build cli_app_game_of_life
