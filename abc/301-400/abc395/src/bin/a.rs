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
    for i in 0..n - 1 {
        if a[i] >= a[i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
