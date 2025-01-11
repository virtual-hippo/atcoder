#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    let ans: usize = (1..10)
        .flat_map(|i| (1..10).map(move |j| (i, j)))
        .map(|(i, j)| i * j)
        .filter(|v| *v != x)
        .sum();
    println!("{}", ans);
}
