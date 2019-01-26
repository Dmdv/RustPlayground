fn main() {
    println!("Hello, world!");


    // if

    let a = 6;

    if a < 10 {
        println!("Less than 10");
    } else {
        println!("Greater than 10");
    }

    // loop

    loop {
        println!("Loop");
        break;
    }

    // while

    let mut num = 5;

    while num > 0 {
        println!("{}!", num);
        num -= 1;
    }

    // for elem

    let x = [1,2,3,4,5];

    for elem in x.iter(){
        println!("Next = {}", elem);
    }

    // Fibonacchi

    let arg = 5;
    let res = fib(arg);

    println!("Fibonacchi from {} = {}", arg, res);
}

fn fib(n:u32) -> u32 {

    if n == 1 || n == 0 {
        return n;
    }

    let mut one_behind = 0;
    let mut two_behind = 1;
    let mut res = 0;

    for _i in 0..n  {
        res = one_behind + two_behind;
        two_behind = one_behind;
        one_behind = res;
    }

    return res;
}
