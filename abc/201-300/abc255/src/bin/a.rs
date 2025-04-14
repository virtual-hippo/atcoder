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
        r: usize,
        c: usize,
        a: [[usize; 2]; 2],
    }

    let ans = a[r - 1][c - 1];
    println!("{}", ans);
}
