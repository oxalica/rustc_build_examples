.PHONY: all clean

all: a
	./a

# Similar to previous one, but we need an extra `-L {dir}` for searching transitive dependencies.
#
# Transitive dependencies are not specified through `--extern` but also need to be loaded,
# since direct dependencies may re-export symbols or expose them in types.
# It use pattern `lib{crate_name}*.rlib` to search them, so make sure that file names match.
# See: https://doc.rust-lang.org/1.0.0/rustc/metadata/loader/index.html#loading-transitive-dependencies
a: a.rs libb1.rlib libb2.rlib
	rustc a.rs -o a --crate-name a --crate-type bin \
		--extern dep1=./libb1.rlib \
		--extern dep2=./libb2.rlib \
		-L .

# Don't forget metadata.
libb1.rlib: b1.rs libc1.rlib
	rustc b1.rs -o libb1.rlib --crate-name b --crate-type rlib \
		--extern deep=./libc1.rlib \
		-C metadata=111

libb2.rlib: b2.rs libc2.rlib
	rustc b2.rs -o libb2.rlib --crate-name b --crate-type rlib \
		--extern deep=./libc2.rlib \
		-C metadata=222

libc1.rlib: c1.rs
	rustc c1.rs -o libc1.rlib --crate-name c --crate-type rlib \
		-C metadata=333

libc2.rlib: c2.rs
	rustc c2.rs -o libc2.rlib --crate-name c --crate-type rlib \
		-C metadata=444

clean:
	-rm -f a lib{b,c}{1,2}.rlib
	