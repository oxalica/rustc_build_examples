.PHONY: all clean

all: a
	./a

a: a.rs libb.so
	rustc a.rs -o a --crate-name a --crate-type bin \
		--extern b=libb.so

# We need to dynamic link `libstd` through `-C prefer-dynamic` to avoid
# linking both dynamic and static std in `a` and `libb.so`.
# See: https://github.com/rust-lang/rust/issues/34909#issuecomment-498893080
libb.so: b.rs
	rustc b.rs -o libb.so --crate-name b --crate-type dylib \
		-C prefer-dynamic

clean:
	-rm -f a libb.so
	