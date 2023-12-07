use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8,
        mut s: Chars,
    }
    for i in 0..s.len() {
        s[i] = (b'A' + (s[i] as u8 - b'A' + n) % 26) as char;
    }
    let ans = s.into_iter().collect::<String>();
    println!("{}", ans);
}
