// From <https://doc.rust-lang.org/1.9.0/book/guessing-game.html>,
// used without permission.

//! Guessing Game from Rust tutorial,
//! updated to use input!() macro.

extern crate rand;

#[macro_use]
extern crate prompted;

use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let n = 100;
    let secret_number = rand::thread_rng().gen_range(1, n+1);

    loop {
        let guess = input!("Please input your guess (1-{}): ", n);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
