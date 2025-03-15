fn main() {
    let s = String::from("Hello, world!");
    let s2 = s; // Ownership moved, 's' is no longer valid
    println!("{}", s2);
}
