all: clean
	rustc --crate-type cdylib src/main.rs
	gcc main.c -I . -L . -lmain

clean:
	rm -f libmain.*
	rm -f a.out
