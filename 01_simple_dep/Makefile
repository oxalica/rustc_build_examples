.PHONY: all clean

all: a
	./a

# Use `--extern {renamed}={path_to_lib}` to specify dependencies.
# See: https://doc.rust-lang.org/1.0.0/rustc/metadata/loader/index.html#the---extern-flag
a: a.rs libc.rlib
	rustc a.rs -o a --crate-type bin \
		--extern dep=./libc.rlib

# Dependency builds to `rlib` file.
# The output name **MUST** in `lib*.rlib` format,
# but we **SHOULD** always use `lib{crate_name}.rlib`.
# See example `03_trans_dep` for details.
libc.rlib: b.rs
	rustc b.rs -o libc.rlib --crate-type rlib

clean:
	-rm -f a b.rlib
