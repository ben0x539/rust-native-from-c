all: main

run: main
	LD_LIBRARY_PATH=. ./main

main: libcounter.a main.c counter.h
	clang -o main -lpthread main.c libcounter.a -ldl -lm

libcounter.a: counter.rs
	rustc -o libcounter.a counter.rs

check: counter-test
	./counter-test

counter-test:
	rustc -o counter-test --test counter.rs

clean:
	rm -f libcounter.a counter-test main
