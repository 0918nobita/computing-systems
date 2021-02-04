BINS := $(patsubst %.s, %.bin, $(wildcard ./*.s))

all: $(BINS)

%.bin: %.o
	ld -o $@ $?

%.o: %.s
	nasm -f elf64 $?

.PHONY: clean
clean:
	rm -f *.bin
