use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        s: Chars,
        // (h,w): (usize, usize),
        // s: Chars,
        // a: [usize; h],
    }
    for i in 0..s.len() {
        if s[i] == '0' {
            print!("1");
        } else {
            print!("0");
        }
    }
}

