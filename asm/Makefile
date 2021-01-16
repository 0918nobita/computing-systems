all: hello print-register funcall

hello: hello.o
	ld -o hello hello.o

hello.o: hello.s
	nasm -f elf64 hello.s

print-register: print-register.o
	ld -o print-register print-register.o

print-register.o: print-register.s
	nasm -f elf64 print-register.s

funcall: funcall.o
	ld -o funcall funcall.o

funcall.o: funcall.s
	nasm -f elf64 funcall.s

.PHONY: clean
clean:
	rm -f *.o
	rm -f hello print-register funcall
