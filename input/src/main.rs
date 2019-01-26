extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    loop {
            
        let mut str = String::new();

        println!("Enter your value:");

        io::stdin().read_line(&mut str).expect("Failed to read the line");

        println!("You input: {}", str);

        let secret_num = rand::thread_rng().gen_range(1, 101);

        println!("Secret number = {}", secret_num);

        let guess:u32 = str.trim().parse().expect("Type in a number");

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
