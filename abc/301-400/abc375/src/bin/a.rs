#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    if n < 3 {
        println!("0");
        return;
    }

    let mut ans = 0;

    for i in 0..n - 2 {
        if s[i] == '#' && s[i + 1] == '.' && s[i + 2] == '#' {
            ans += 1;
        }
    }
    println!("{}", ans);
}
