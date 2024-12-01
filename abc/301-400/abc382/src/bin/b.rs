#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        mut s: Chars,
    }
    let mut cnt = 0;
    for i in (0..n).rev() {
        if cnt < d && s[i] == '@' {
            s[i] = '.';
            cnt += 1
        }
    }
    for i in 0..n {
        print!("{}", s[i]);
    }
}
