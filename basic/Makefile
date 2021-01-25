BINS := $(patsubst %.bas, %.bin, $(wildcard *.bas))

all: $(BINS)

%.bin: %.bas
	cargo run --bin compiler -- $?

.PHONY: clean
clean:
	rm -f *.bin *.o *.s
