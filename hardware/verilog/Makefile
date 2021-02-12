all: hello.vvp $(patsubst %.v, %.vvp, $(wildcard ./*_test.v))

.PHONY: tests
tests: and_test and16_test or_test

hello.vvp: hello.v
	iverilog -o hello.vvp hello.v

and_test.vvp: and_test.v and.v
	iverilog -o and_test.vvp -s ANDTEST and_test.v and.v

.PHONY: and_test
and_test: and_test.vvp
	vvp and_test.vvp

and16_test.vvp: and16_test.v and16.v
	iverilog -o and16_test.vvp -s AND16TEST and16_test.v and16.v

.PHONY: and16_test
and16_test: and16_test.vvp
	vvp and16_test.vvp

or_test.vvp: or_test.v or.v
	iverilog -o or_test.vvp -s ORTEST or_test.v or.v

.PHONY: or_test
or_test: or_test.vvp
	vvp or_test.vvp

.PHONY: clean
clean:
	rm -f *.vvp *.vcd
