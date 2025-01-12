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
    }

    let ans = f(n);
    println!("{}", ans);
}

fn f(x: usize) -> usize {
    if x == 0 {
        1
    } else {
        x * f(x - 1)
    }
}
