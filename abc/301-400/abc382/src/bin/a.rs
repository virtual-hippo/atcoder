#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        s: Chars,
    }
    let cnt = s.iter().filter(|c| **c == '@').count();
    let ans = n - (cnt - d);
    println!("{}", ans);
}
