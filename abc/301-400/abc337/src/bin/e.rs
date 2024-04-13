use proconio::input;
use proconio::marker::Chars;
use proconio::source::line::LineSource;
use std::io::{self, BufReader};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {
        n: usize,
    }
    let m = n - 1;
    println!("{}", m);
    for i in 0..m {
        println!("2 {} {}", i + 1, i + 2);
    }
    input! {
        s: Chars,
    }
    if s[0] == '1' && s[1] == '0' {
        println!("1");
        return;
    }
    if s[m - 2] == '0' && s[m - 1] == '1' {
        println!("{}", m);
        return;
    }
    for i in 1..m {
        if s[i] == '1' && s[i - 1] == '1' {
            println!("{}", i + 1);
            return;
        }
    }
}
