
.PHONY: clean lib check test bench all

all: clean lib check

clean:
	-rm librand*.so test-rand

check: test bench

bench: test-rand
	./test-rand --bench $(TESTNAME)

test: test-rand
	./test-rand $(TESTNAME)

test-rand: *.rs */*rs
	rustc --opt-level=3 --test mod.rs -o test-rand

lib: librand*.so

librand*.so: *.rs */*rs
	rustc --opt-level=3 mod.rs
