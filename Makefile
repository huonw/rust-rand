RUSTC=rustc
RUSTFLAGS=--opt-level=3

.PHONY: clean lib check test bench all run

all: lib

run: bench/run

bench/run: bench/run.rs
	$(RUSTC) --opt-level=3 -L . bench/run.rs

clean:
	-rm librand*.so test-rand

check: test bench

bench: lib-bench ext-bench

lib-bench: test-rand
	./test-rand --bench $(TESTNAME)

ext-bench: lib
	bench/run.sh $(RUSTC)

test: test-rand
	./test-rand $(TESTNAME)

test-rand: *.rs */*rs
	$(RUSTC) $(RUSTFLAGS) --test mod.rs -o test-rand

lib: librand*.so

librand*.so: *.rs */*rs
	$(RUSTC) $(RUSTFLAGS) mod.rs
