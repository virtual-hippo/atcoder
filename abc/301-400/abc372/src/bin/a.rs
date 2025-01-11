#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans: String = s.chars().filter(|&v| v != '.').collect();
    println!("{}", ans);
}
