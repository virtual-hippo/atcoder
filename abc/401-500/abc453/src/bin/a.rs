#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{input, marker::*};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::*;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let pos = s.iter().position(|&ch| ch != 'o').unwrap_or(n);

    let ans = if pos == n { format!("") } else { (pos..n).map(|i| s[i]).collect() };
    println!("{}", ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
