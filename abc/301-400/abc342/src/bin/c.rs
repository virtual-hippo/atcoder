use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
        q: usize,
    }
    let mut state = (0..26).map(|i| i + 97).collect::<Vec<u8>>();
    for _ in 0..q {
        input! {
            c: char,
            d: char,
        }
        for i in 0..26 {
            if state[i] == c as u8 {
                state[i] = d as u8;
            }
        }
    }
    let ans = s
        .iter()
        .map(|&ch| state[(ch as u8 - 97) as usize] as char)
        .collect::<String>();
    println!("{}", ans);
}
