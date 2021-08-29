BINS := $(patsubst %.ll, %.bin, $(wildcard ./*.ll))

all: $(BINS)

%.bin: %.s
	gcc -no-pie -o $@ $?

%.s: %.bc
	llc $?

%.bc: %.ll
	llvm-as $?

.PHONY: clean
clean:
	rm -f *.bc *.bin *.s
