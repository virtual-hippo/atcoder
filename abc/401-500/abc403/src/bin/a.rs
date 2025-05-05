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
        a: [usize; n],


    }

    let ans = (0..n).filter(|i| i % 2 == 0).map(|i| a[i]).sum::<usize>();

    println!("{}", ans);
}
