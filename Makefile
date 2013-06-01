RUSTC=rustc
RUSTFLAGS=--opt-level=3

.PHONY: clean lib check test bench all

all: lib

clean:
	-rm librand*.so test-rand

check: test bench

bench: lib-bench ext-bench

lib-bench: test-rand
	./test-rand --bench $(TESTNAME)

ext-bench: lib
	bench/run.sh

test: test-rand
	./test-rand $(TESTNAME)

test-rand: *.rs */*rs
	$(RUSTC) $(RUSTFLAGS) --test mod.rs -o test-rand

lib: librand*.so

librand*.so: *.rs */*rs
	$(RUSTC) $(RUSTFLAGS) mod.rs
