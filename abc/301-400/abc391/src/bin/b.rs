#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    let ans = iproduct!(0..n - m + 1, 0..n - m + 1)
        .find(|&(a, b)| iproduct!(0..m, 0..m).all(|(i, j)| s[a + i][b + j] == t[i][j]))
        .expect("No matching submatrix found");

    println!("{} {}", ans.0 + 1, ans.1 + 1);
}
