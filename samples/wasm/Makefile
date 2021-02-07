BINS := $(patsubst %.wat, %.wasm, $(wildcard ./*.wat))

all: $(BINS)

%.wasm: %.wat
	wat2wasm $?

.PHONY: clean
clean:
	rm -f *.wasm
