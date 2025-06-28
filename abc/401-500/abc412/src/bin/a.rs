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
        ab: [(usize, usize); n],
    }

    let ans = ab.iter().filter(|&&(a, b)| a < b).count();
    println!("{}", ans);
}
