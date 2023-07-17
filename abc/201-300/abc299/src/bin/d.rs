use proconio::input;
use proconio::source::line::LineSource;
use std::io::{self, BufReader};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {
        n: usize,
    }

    let (mut left, mut right) = (1, n);

    let mut i = 0;
    while right - left > 1 && i < 20 {
        let mid = (left + right) / 2;
        println!("? {}", mid);
        input! {
            si: i32,
        }
        if si == 0 {
            left = mid;
        } else {
            right = mid;
        }
        i += 1;
    }
    println!("! {}", left);
}

