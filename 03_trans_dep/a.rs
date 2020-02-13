/// Transitive dependencies.
///
/// Dependency graph:
///  /-- b1 -- c1
/// a
///  \-- b2 -- c2
///
/// Equivalent `Cargo.toml`:
///
/// Crate `a`:
/// ```
/// [dependencies]
/// dep1 = { package = "b", version = "1" }
/// dep2 = { package = "b", version = "2" }
/// ```
///
/// Crate `b` version 1:
/// ```
/// [dependencies]
/// deep = { package = "c", version = "1" }
/// ```
///
/// Crate `b` version 2:
/// ```
/// [dependencies]
/// deep = { package = "c", version = "2" }
/// ```

fn main() {
    println!("{}", dep1::foo() + dep2::foo());
}
