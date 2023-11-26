use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let pos = s.iter().position(|&ch| ch == '1').unwrap();
    if pos % 2 == 0 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
