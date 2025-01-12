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
        x: usize,
        k: usize,
    }

    let mut ans = x as f64;

    for i in 0..k {
        let base_number = 10_usize.pow(i as u32 + 1) as f64;
        ans = (ans / base_number).round() * base_number;
    }

    let ans = ans as usize;

    println!("{}", ans);
}
