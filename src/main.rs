// use std::io;
// use rand::Rng;
// fn main() {
//     println!("Guess the number!");
//
//     println!("Please input your guess.");
//
//     let mut guess = String::new();
//
//     io::stdin().read_line(&mut guess).expect("Failed to read line");
//
//     println!("You guessed: {guess}");
// }


// use std::io;
// use rand::Rng;
// fn main() {
//     println!("Guess the number!");
//
//     // 奇怪的语法
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//
//     println!("The secret number is: {}", secret_number);
//
//     println!("Please input your guess.");
//
//     let mut guess = String::new();
//
//     io::stdin().read_line(&mut guess).expect("Failed to read line");
//
//     println!("You guessed: {guess}");
// }

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // 生成一个随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // 用户输入
    let mut guess_str = String::new();
    io::stdin().read_line(&mut guess_str).expect("Failed to read line");

    let mut guess: u32 = guess_str.trim().parse().expect("Please type a number!");
    loop {
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
        guess_str.clear();
        io::stdin().read_line(&mut guess_str).expect("Failed to read line");
        guess = guess_str.trim().parse().expect("Please type a number!");
    }
}