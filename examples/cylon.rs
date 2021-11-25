// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! ASCII-art "Cylon" display.

//! This is something we used to write in the 1980s to run
//! on our glass TTYs.

use std::str;
use std::thread::sleep;
use std::time::Duration;

use prompted::prompt;

const WIDTH: usize = 7;

pub fn main() {
    let mut chars = [b' '; WIDTH];
    chars[0] = b'*';
    let mut posn = 0;
    let mut dirn = -1isize;
    loop {
        chars[posn] = b' ';
        if posn == 0 || posn == WIDTH - 1 {
            dirn = -dirn
        };
        posn = (posn as isize + dirn) as usize;
        chars[posn] = b'*';
        prompt!("{}\r", str::from_utf8(&chars).unwrap());
        sleep(Duration::from_millis(100));
    }
}
