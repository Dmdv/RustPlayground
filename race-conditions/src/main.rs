fn main() {
    println!("Hello, world!");
    let refers_nothing = dangle();

    println!("{}", refers_nothing);
}

fn dangle() -> String {
    let s = String::from("String");
    s
}
