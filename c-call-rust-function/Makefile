all:
	rustc rusttest.rs
	gcc -L. -Wall -o main main.c -lrusttest

clean:
	rm -f main librusttest.so

run_c:
	LD_LIBRARY_PATH=. ./main