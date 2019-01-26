fn main() {
    println!("Hello, world!");
    let s = String::from("Hello, World!");
    let slice = &s[0..5];
    println!("{}", slice);
}
