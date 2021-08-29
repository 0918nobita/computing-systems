main.bin: main.o concat-strs.o
	gcc -no-pie -o $@ $?

main.o: main.c
	gcc -c $?

concat-strs.o: concat-strs.s
	nasm -f elf64 $?

.PHONY: clean
clean:
	rm -f *.bin *.o
