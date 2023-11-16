![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/BartMassey/prompted/actions/workflows/rust.yml/badge.svg)](https://github.com/BartMassey/prompted/actions/workflows/rust.yml)
[![crates-io](https://img.shields.io/crates/v/prompted.svg)](https://crates.io/crates/prompted)
[![api-docs](https://docs.rs/prompted/badge.svg)](https://docs.rs/prompted)
[![dependency-status](https://deps.rs/repo/github/BartMassey/prompted/status.svg)](https://deps.rs/repo/github/BartMassey/prompted)

# prompted: simple prompting and input
Copyright © Bart Massey 2017 (Version 0.2.8)

This crate provides macros for easy non-newline-terminated
flushed printing, and for input line reading. These macros
are intended for new Rust users and for folks who need no
more for simple applications.

## Example

Here's an example adapted from the "Guessing Game" example
in [*The Rust Programming
Language*](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

```rust
use std::cmp::Ordering;
use prompted::input;

fn main() {
    println!("Guess the number!");

    let n = 100;
    let secret_number = 37;

    loop {
        let guess = input!("Please input your guess (1-{}): ", n);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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
```


# License

This work is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.

# Acknowledgments

Thanks to the `cargo-readme` crate for generation of this `README`.
