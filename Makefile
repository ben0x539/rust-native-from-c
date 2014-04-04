all: main

run: main
	LD_LIBRARY_PATH=. ./main

main: libcounter.so main.c counter.h
	clang -o main -lpthread libcounter.so main.c

libcounter.so: counter.rs
	rustc -o libcounter.so counter.rs

check: counter-test
	./counter-test

counter-test:
	rustc -o counter-test --test counter.rs

clean:
	rm -f libcounter.so counter-test main
