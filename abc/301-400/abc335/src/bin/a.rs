use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    for i in 0..s.len() {
        if i != s.len() - 1 {
            print!("{}", s[i]);
        } else {
            print!("{}", 4);
        }
    }
}
