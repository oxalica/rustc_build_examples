/// Simple dependency with renaming.
///
/// Equivalent `Cargo.toml`:
/// ```
/// [dependencies]
/// dep = { package = "b", version = "*" }
/// ```

fn main() {
    println!("{}", dep::foo());
}
