all:
	gcc -c -Wall -Werror -fpic test.c
	gcc -shared -o libtest.so test.o
	gcc -L. -Wall -o main main.c -ltest
	rustc -L. rust-call-c-function.rs

clean:
	rm -f libtest.so main test.o rust-call-c-function

run_c:
	LD_LIBRARY_PATH=. ./main

run_rust:
	LD_LIBRARY_PATH=. ./rust-call-c-function