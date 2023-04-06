#![doc(html_root_url = "https://docs.rs/prompted/0.2.6")]
// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Prompting macros.
//!
//! These are eventually intended to go into the standard
//! library macros. At that point, parts of them will need
//! to be reimplented for efficiency and compatibility.

use std::io::{stderr, stdin, stdout, Write};

/// Flush standard output. Intended primarily for use by
/// macros.
///
/// # Panics
///
/// Panics if writing to [stdout] fails.
pub fn flush() {
    if let Err(e) = stdout().flush() {
        panic!("Failed to flush stdout: {}", e);
    }
}

/// Flush standard error. Intended primarily for use by
/// macros.
///
/// # Panics
///
/// Panics if writing to [stderr] fails.
pub fn eflush() {
    if let Err(e) = stderr().flush() {
        panic!("Failed to flush stderr: {}", e);
    }
}

/// Read a line from standard input. Removes a trailing
/// newline if present. Intended primarily for use by
/// macros.
///
/// # Panics
///
/// Panics if reading from [stdin] fails.
pub fn read_line() -> String {
    let mut buf = String::new();
    match stdin().read_line(&mut buf) {
        Err(e) => panic!("Failed to read stdin: {}", e),
        _ => match buf.pop() {
            None => (),
            Some(c) => {
                if c != '\r' && c != '\n' {
                    buf.push(c)
                }
            }
        },
    };
    buf
}

/// Same functionality as [print!()] except that [stdout]
/// is flushed at the end.
///
/// As with [print!()], the multi-argument form of this
/// macro supports the [format!()] syntax for building a
/// string. With no arguments, only the flush is performed.
///
/// # Panics
///
/// Panics if writing to [stdout] fails.
///
/// # Examples
///
/// ```
/// # use prompted::prompt;
/// # pub fn main() {
/// prompt!();
/// prompt!("Pick a number between 1 and {}: ", 10);
/// # }
/// ```
#[macro_export]
macro_rules! prompt {
    () => ($crate::flush());
    ($($arg:tt)*) => ({print!($($arg)*);$crate::flush()});
}

/// Same functionality as [prompt!()] except using [stderr]
/// instead of [stdout].
///
/// # Panics
///
/// Panics if writing to [stderr] fails.
#[macro_export]
macro_rules! eprompt {
    () => ($crate::eflush());
    ($($arg:tt)*) => ({eprint!($($arg)*);$crate::eflush()});
}

/// If a [format!()] describing a prompt is present, print it
/// on [stdout] and then flush. Then read a line from
/// [stdin] and return it after removing the line ending.
///
/// # Panics
///
/// Panics if reading from [stdin] or writing to [stdout] fails.
///
/// # Examples
///
/// ```
/// # use prompted::input;
/// # pub fn main() {
/// let m = 10;
/// let guess = input!("Pick a number between 1 and {}: ", m);
/// match guess.parse::<isize>() {
///     Ok(n) => if n >=1 && n <= m {
///         println!("Thank you for choosing {}", n)
///     } else {
///         println!("You failed arithmetic with {}", n)
///     },
///     Err(e) => println!("Not even a number. {}?", e)
/// }
/// # }
/// ```
#[macro_export]
macro_rules! input {
    () => ($crate::read_line());
    ($($arg:tt)*) =>
        ({print!($($arg)*);$crate::flush();$crate::read_line()});
}
