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

    for i in 0..n {
        let k = i + 1;

        if (k) % 3 == 0 {
            print!("x");
        } else {
            print!("o");
        }
    }
}
