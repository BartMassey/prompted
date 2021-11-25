// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Show phases of process.

//! This demo simulates the display of long-running
//! process state on a single line.

use std::thread::sleep;
use std::time::Duration;

use prompted::prompt;

fn f_pre() {
    sleep(Duration::from_millis(1000))
}

fn f_op() {
    sleep(Duration::from_millis(2000))
}

fn f_post() {
    sleep(Duration::from_millis(500))
}

pub fn main() {
    let phases: &[(usize, &'static str, fn())] =
        &[(1, "pre", f_pre), (2, "op", f_op), (3, "post", f_post)];
    let mut last_len = 0;
    for &(n, name, f) in phases {
        for _ in 0..last_len {
            print!(" ")
        }
        let message = format!("{}: {}", n, name);
        prompt!("\r{}", message);
        f();
        last_len = message.len();
        print!("\r")
    }
    println!()
}
