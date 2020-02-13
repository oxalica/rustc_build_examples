/// Using two different versions of the same crate.
///
/// Equivalent `Cargo.toml`:
/// ```
/// [dependencies]
/// dep1 = { package = "b", version = "1" }
/// dep2 = { package = "b", version = "2" }
/// ```

fn main() {
    println!("{}", dep1::foo() + dep2::foo());
}
