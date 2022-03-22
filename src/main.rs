#![warn(clippy::all, clippy::pedantic)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number.");

    let secret_number: u8 = rand::thread_rng().gen_range(1..101);

    println!("Please enter an input.");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Unknown input.");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
            Ordering::Greater => println!("Too large!"),
        }
    }
}
