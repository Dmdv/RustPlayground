fn main() {
    println!("Hello, world!");
    another_function();
}


fn another_function(){
    println!("From another function");
    another_function2(5);
    let result = sum(3, 5);
    println!("Sum of 3 and 5 equals = {}", result);
}

fn another_function2(i:u8){
    println!("From another function2 x = {}", i);
}

fn sum(a:i32, b:i32) -> i32 {
    let b2 = a + b;
    b2 * 1000
}