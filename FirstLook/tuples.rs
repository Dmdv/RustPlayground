fn main() {

    let tup: (i32, f64, u8) = (500, 3.1, 1);

    let (x,y,z) = tup;

    println!("Value of x = {}", x);
    println!("Value of y = {}", y);
    println!("Value of z = {}", z);

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("Value of a = {}, b = {}, c = {}", a, b, c);
}