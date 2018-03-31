CPPARG=--std=c++1z
ARG=-g3

COBJS := $(patsubst %.c,%.o,$(wildcard sundown/*.c))

.PHONY: clean clean_main all

all: wrapper.o main

main: main.o $(COBJS)
	g++ -o $@ $^ $(CPPARG) $(ARG)

wrapper.o: wrapper.c
	gcc -c $^ $(ARG)

main.o: main.cpp
	g++ -c $^ $(CPPARG) $(ARG)
	
sundown/%.o: sundown/%.c
	gcc -o $@ -c $^ $(ARG)

clean_main:
	rm main

clean: clean_main
	rm *.o
	rm sundown/*.o
