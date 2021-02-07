main.bin: main.o concat-strs.o
	gcc -no-pie -lSystem -o $@ $?

main.o: main.c
	gcc -c -no-pie $?

concat-strs.o: concat-strs.s
	nasm -f macho64 $?

.PHONY: clean
clean:
	rm -f *.bin *.o
