.PHONY: all clean

all: a
	./a

# Just the same to the previous one.
a: a.rs libb1.rlib libb2.rlib
	rustc a.rs -o a --crate-name a --crate-type bin \
		--extern dep1=./libb1.rlib \
		--extern dep2=./libb2.rlib

# We need to add `-C metadata={distinct_string}` to avoid link errors about duplicated symbols.
# See: https://doc.rust-lang.org/1.0.0/rustc/metadata/loader/index.html#frobbing-symbols
libb1.rlib: b1.rs
	rustc b1.rs -o libb1.rlib --crate-name b --crate-type rlib \
		-C metadata=111

# The same `--crate-name` but different metadata.
libb2.rlib: b2.rs
	rustc b2.rs -o libb2.rlib --crate-name b --crate-type rlib \
		-C metadata=222

clean:
	-rm -f a libb{1,2}.rlib
	
