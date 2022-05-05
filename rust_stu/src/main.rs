use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..101); // (1..=100)

    println!("Guess the number"); 

    loop {
        println!("Please input your guess."); 

        // let 定义变量
        // mut 使变量可变
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fail to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("rand number is {}", secret_number); 
        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
