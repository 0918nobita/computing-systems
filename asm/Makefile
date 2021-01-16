NASM = nasm -f elf64

all: hello print-register funcall string print-string

hello: hello.o
	ld -o hello hello.o

hello.o: hello.s
	$(NASM) hello.s

print-register: print-register.o
	ld -o print-register print-register.o

print-register.o: print-register.s
	$(NASM) print-register.s

funcall: funcall.o
	ld -o funcall funcall.o

funcall.o: funcall.s
	$(NASM) funcall.s

string: string.o
	ld -o string string.o

string.o:
	$(NASM) string.s

print-string: print-string.o
	ld -o print-string print-string.o

print-string.o: print-string.s
	$(NASM) print-string.s

.PHONY: clean
clean:
	rm -f *.o
	rm -f hello print-register funcall string print-string
