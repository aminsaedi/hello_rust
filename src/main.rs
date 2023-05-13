use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the \"Guess the number!\"");

    loop { 
        println!("Please enter your guess.");

        let mut guess = String::new();

        let mut generator = rand::thread_rng();

        let secret_number = generator.gen_range(1..=10);

        println!("Machine generated number is: {}", secret_number);

        let count = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            },
        }
    }
} 
