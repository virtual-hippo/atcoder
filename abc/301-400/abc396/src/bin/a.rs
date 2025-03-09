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

    for i in 0..n - 2 {
        if a[i] == a[i + 1] && a[i + 1] == a[i + 2] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
