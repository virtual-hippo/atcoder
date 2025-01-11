#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans = (a + b) * (a + b);
    println!("{}", ans);
}
