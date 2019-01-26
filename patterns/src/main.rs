fn main() {
    let s = String::from("Rum");

    let c = match s.chars().nth(4) {
        Some(expr) => expr.to_string(),
        None => "No character found".to_string()
    };

    println!("Result = '{}'", c);

    let integer_value = 5;

    match integer_value {
        1 => println!("One"),
        _ => ()
    }
}
