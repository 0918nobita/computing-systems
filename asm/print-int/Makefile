main.bin: main.o print-int.o
	gcc -no-pie -o main.bin main.o print-int.o

main.o: main.c
	gcc -c main.c

print-int.o: print-int.s
	nasm -f elf64 print-int.s

.PHONY: clean
clean:
	rm -f *.bin *.o
