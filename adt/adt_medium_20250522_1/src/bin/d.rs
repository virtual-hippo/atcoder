#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let b = a.iter().enumerate().sorted_by(|a, b| a.1.cmp(b.1)).collect_vec();

    let ans = b[n - 2].0 + 1;
    println!("{}", ans);
}
