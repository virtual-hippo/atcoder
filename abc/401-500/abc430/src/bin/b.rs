use itertools::*;
use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n]
    }

    let ans = iproduct!(0..(n - m + 1), 0..(n - m + 1))
        .map(|(i, j)| iproduct!(i..(i + m), j..(j + m)).map(|(ii, jj)| s[ii][jj]).collect::<String>())
        .collect::<HashSet<String>>()
        .len();

    println!("{}", ans);
}
