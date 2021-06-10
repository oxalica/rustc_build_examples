.PHONY: all clean

all: a
	./a

# We should `-C prefer-dynamic` for the whole chain, or we'll mix static and dynamic libstd, which leads to an error.
# `-L {dir}` is still required for transparent dependencies.
a: a.rs libb1.so libb2.so
	rustc a.rs -o a --crate-name a --crate-type bin \
		--extern dep1=./libb1.so \
		--extern dep2=./libb2.so \
		-C prefer-dynamic \
		-L .

# Don't forget metadata.
libb1.so: b1.rs libc1.so
	rustc b1.rs -o libb1.so --crate-name b --crate-type dylib \
		-C prefer-dynamic \
		--extern deep=./libc1.so \
		-C prefer-dynamic \
		-C metadata=111

libb2.so: b2.rs libc2.so
	rustc b2.rs -o libb2.so --crate-name b --crate-type dylib \
		--extern deep=./libc2.so \
		-C prefer-dynamic \
		-C metadata=222

libc1.so: c1.rs
	rustc c1.rs -o libc1.so --crate-name c --crate-type dylib \
		-C prefer-dynamic \
		-C metadata=333

libc2.so: c2.rs
	rustc c2.rs -o libc2.so --crate-name c --crate-type dylib \
		-C prefer-dynamic \
		-C metadata=444

clean:
	-rm -f a lib{b,c}{1,2}.so

