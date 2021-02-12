OBJS := $(patsubst %.v, %.vvp, $(wildcard ./*.v))

all: $(OBJS)

%.vvp: %.v
	iverilog -o $@ $?

.PHONY: clean
clean:
	rm -f *.vvp *.vcd
