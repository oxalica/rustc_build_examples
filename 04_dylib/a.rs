fn main() {
    assert!(b::bar::<i32>().is_none());
    println!("{}", b::foo());
}
