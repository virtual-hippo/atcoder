#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    if n == 2 {
        println!("Yes");
        return;
    }

    for i in 0..n - 2 {
        if a[i] * a[i + 2] != a[i+1] * a[i+1] {
            println!("No");
            return;
        }
    }
    println!("Yes");

}
