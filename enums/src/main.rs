#[derive(Debug)]
enum IPAddressKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x:u32, y:u32}, // anonimous struct
    Write (String),
    ChangeColor(i32,i32,i32)
}

impl Message {
    fn call(&self) {
        println!("Inside call");
    }
}

fn main() {

    let v4 = IPAddressKind::V4;
    let v6 = IPAddressKind::V6;

    //println!("V4 = {}", v4.to_string());
    // println!("V6 = {}", v6);

    let loc4 = IPAddressKind::V4(String::from("127.0.0.1"));
    let loc6 = IPAddressKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Sample string"));
    m.call();

    //let opt = Option<String>::Some(String::from("This is an option value"));

    // println!("V4 = {}", loc4);
    // println!("V6 = {}", loc6);
}
