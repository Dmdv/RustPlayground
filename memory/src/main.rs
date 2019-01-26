fn main() {

    let mut s = String::from("Hello");
    s.push_str(", World!");

    // shallow copy
    let s2 = s;
    // it will fail, move pointer
    // println!("{}", s);

    println!("{}", s2);
    takes_ownership(s2);
    //it will fail 
    //println!("{}", s2);

    let s3 = String::from("Hello3");
    takes_ownership2(&s3);
    println!("{}", s3); // you can use it after passing

    let mut s4 = String::from("Hello4 ");
    change_variable(&mut s4);
    println!("{}", s4);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn takes_ownership2(s: &String) {
    println!("{}", s);
}

fn change_variable(s: &mut String) {
    s.push_str("New value");
    println!("{}", s);
}