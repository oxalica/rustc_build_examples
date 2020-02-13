.PHONY: all clean

all: a
	./a

# Simple.
a: a.rs
	rustc a.rs -o a --crate-name a --crate-type bin

clean:
	-rm -f a
