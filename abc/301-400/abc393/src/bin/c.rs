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
        _n: usize,
        m: usize,
    }

    let mut set = FxHashSet::default();

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        let a = a - 1;
        let b = b - 1;
        let (a, b) = if a > b { (b, a) } else { (a, b) };

        if a == b {
            continue;
        }
        if set.contains(&(a, b)) {
        } else {
            set.insert((a, b));
        }
    }

    println!("{}", m - set.len());
}
