#![doc(html_root_url = "https://docs.rs/prompted/0.3.0-pre")]
// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

#![allow(clippy::needless_doctest_main)]

/*!
This crate provides macros for easy non-newline-terminated
flushed printing, and for input line reading. These macros
are intended for new Rust users and for folks who need no
more for simple applications.

# Example

Here's an example adapted from the "Guessing Game" example
in [*The Rust Programming
Language*](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

```no_run
use std::cmp::Ordering;
use prompted::inputln;

fn main() {
    println!("Guess the number!");

    let n = 100;
    let secret_number = 37;

    loop {
        let guess = inputln!("Please input your guess (1-{}): ", n);

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

*/

use std::io::{stdin};

/// Read a line from `stdin`. The line ending (LF or CRLF)
/// is not returned.
///
/// # Panics
///
/// Panics if reading from `stdin` fails.
pub fn inputln() -> String {
    let mut buf = String::new();
    let _nchars = stdin().read_line(&mut buf).unwrap();
    if buf.ends_with('\n') {
        buf.pop();
        if buf.ends_with('\r') {
            buf.pop();
        }
    }
    buf
}

/// Error produced by `readln()`.
#[derive(Debug)]
pub enum ReadlnError {
    /// IO error during reading.
    Io(std::io::Error),
    /// Error in converting to [String].
    Utf8(std::string::FromUtf8Error),
}

impl std::fmt::Display for ReadlnError {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadlnError::Io(e) => write!(f, "readln: io error: {e}"),
            ReadlnError::Utf8(e) => write!(f, "readln: utf8 error: {e}"),
        }
   }
}

impl std::error::Error for ReadlnError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadlnError::Io(e) => Some(e),
            ReadlnError::Utf8(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for ReadlnError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<std::string::FromUtf8Error> for ReadlnError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::Utf8(error)
    }
}

/// Read a line from the given reader, one byte at a time,
/// until a LF byte is found. The reader is expected to
/// produce UTF-8. The line ending is preserved.
pub fn readln(reader: impl std::io::Read) -> Result<String, ReadlnError> {
    let mut buf = Vec::new();
    for b in reader.bytes() {
        let b = b?;
        buf.push(b);
        if b == b'\n' {
            break;
        }
    }
    let buf = String::from_utf8(buf)?;
    Ok(buf)
}

/// Same functionality as [print!()] except that `stdout`
/// is flushed at the end.
///
/// As with [print!()], the multi-argument form of this
/// macro supports the [format!()] syntax for building a
/// string. With no arguments, only the flush is performed.
///
/// # Panics
///
/// Panics if writing to `stdout` fails.
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
    () => {
        use ::std::io::Write;
        ::std::io::stdout().flush().unwrap()
    };
    ($($arg:tt)*) => {{
        use ::std::io::Write;
        print!($($arg)*);
        ::std::io::stdout().flush().unwrap()
    }};
}

/// Same functionality as [prompt!()] except using `stderr`
/// instead of `stdout`.
///
/// # Panics
///
/// Panics if writing to `stderr` fails.
#[macro_export]
macro_rules! eprompt {
    () => {
        use ::std::io::Write;
        ::std::io::stderr().flush().unwrap()
    };
    ($($arg:tt)*) => {{
        use ::std::io::Write;
        eprint!($($arg)*);
        ::std::io::stderr().flush().unwrap()
    }};
}

/// Write to the writer given as the first argument.  The
/// writer is flushed after a successful write. Returns
/// [std::io::Result(())].
///
/// As with [print!()], the multi-argument form of this
/// macro supports the [format!()] syntax for building a
/// string. Without format arguments, only the flush is performed.
///
/// # Examples
///
/// ```
/// # use prompted::write_prompt;
/// # pub fn main() {
/// # use std::io::stdout;
/// write_prompt!(stdout(), "Pick a number between 1 and {}: ", 10).unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! write_prompt {
    ($writer:expr) => {
        use ::std::io::Write;
        $writer.flush()
    };
    ($writer:expr, $($arg:tt)*) => {{
        use ::std::io::Write;
        write!($writer, $($arg)*)
            .and_then(|()| ::std::io::stderr().flush())
    }};
}

/// If a [format!()] describing a prompt is present, print it
/// on `stdout` and then flush. Then read a line from
/// `stdin` and return it after removing the line ending.
///
/// # Panics
///
/// Panics if reading from `stdin` or writing to `stdout` fails.
///
/// # Examples
///
/// ```
/// # use prompted::inputln;
/// # pub fn main() {
/// let m = 10;
/// let guess = inputln!("Pick a number between 1 and {}: ", m);
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
macro_rules! inputln {
    () => {
        $crate::inputln().unwrap()
    };
    ($($arg:tt)*) => {{
        use ::std::io::Write;
        print!($($arg)*);
        ::std::io::stdout().flush().unwrap();
        $crate::inputln()
    }};
}
