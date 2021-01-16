main: main.o string-length.o
	gcc -o main main.o string-length.o

main.o: main.c
	gcc -c main.c

string-length.o: string-length.s
	nasm -f elf64 string-length.s

.PHONY: clean
clean:
	rm -f main main.o string-length.o
