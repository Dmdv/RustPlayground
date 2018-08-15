use std::io;

fn main() {
    println!("Угадай число");
    println!("Введите предположение");
    
    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("Не удалось прочитать строку");

    println!("Ваша попытка: {}", guess);

}