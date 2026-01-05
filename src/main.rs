// ==================== capter02 ==================== //

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

// use std::cmp::Ordering;
// use std::io;
// use rand::Rng;
//
// fn main() {
//     println!("Guess the number!");
//     // 生成一个随机数
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     loop {
//         println!("Please input your guess.");
//         // 用户输入
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("Failed to read line");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         println!("You guessed: {}", guess);
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }


// ==================== capter03 ==================== //


// fn main() {
//     // mut 关键字代表变量是否可变
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }


// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// fn main() {
//     println!("Three Hours in Seconds is {THREE_HOURS_IN_SECONDS}");
// }


// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let x = 2.0;
//     let y: f32 = 3.0;
//     println!("x: {}, y: {}", x, y);
// }

// fn main() {
//     // addition
//     let sum = 5 + 10;
//
//     // subtraction
//     let difference = 95.5 - 4.3;
//
//     // multiplication
//     let product = 4 * 30;
//
//     // division
//     let quotient = 56.7 / 32.2;
//
//     let truncated = -5 / 3;
//
//     let remainder = 43 % 5;
//
//     let t = true;
//
//     let f: bool = false;
//
//     println!("The sum is {}", sum);
//     println!("The difference is {}", difference);
//     println!("The product is {}", product);
//     println!("The quotient is {}", quotient);
//     println!("The truncated is {}", truncated);
//     println!("The remainder is {}", remainder);
//     println!("The quotient is {}", quotient);
//     println!("The truncated is {}", truncated);
//     println!("The remainder is {}", remainder);
//     println!("The true is {}", t);
//     println!("The false is {}", f);
// }

// fn main() {
//     // 元组要指定每一个元素的类型吗
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//
//     let tt = (500, 1, 2, -1, 3.22, &tup);
//
//
//     println!("{:#?}", tt);
// }

// fn main() {
//     let a = [12, 3, 4];
//     println!("{:#?}", a);
//
//
//     let a: [i32; 6] = [1; 6];
//
//     for i in &a {
//         print!("{}, ", i);
//     }
// }

// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     println!("Please enter an array index.");
//
//     let mut index = String::new();
//
//     std::io::stdin().read_line(&mut index).expect("Failed to read line");
//
//     // 不明白这里为什么数组的索引里面的变量必须得是 usize类型
//     let index: usize = index.trim().parse().expect("Please type a number");
//
//     let element = a[index];
//
//     println!("The value of the element at index {index} is: {element}");
// }


// fn main() {
//     println!("Hello, world!");
//     another_function(&mut 32);
// }
//
// // fn another_function() {
// //     println!("Another function.");
// // }
//
// fn another_function(x: &mut i32) {
//     *x = 44;
//     println!("Another function. x = {}", x);
// }


// fn main() {
//     let y = {
//         let x = 5;
//         x + 1;
//     };
//     println!("y is {}", y == ());
// }


// fn five() -> i32 {
//     5
// }
//
// fn main() {
//     let x = five();
//     println!("The value of x is: {}", x);
// }


// fn main() {
//     let x = plus_one(5);
//
//     println!("The value of x is: {:?}", x);
// }
//
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }


// ==================== capter04 ==================== //

// hello, world

// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.

// fn main() {
//     let lucky_number = 7; // I'm feeling lucky today
// }

// fn main() {
//     // I'm feeling lucky today
//     let lucky_number = 7;
// }

// ==================== capter05 ==================== //

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}