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
        d: [usize; n-1],
    }
    for i in 0..n - 1 {
        let mut s = 0;
        for j in i..n - 1 {
            s += d[j];
            print!("{} ", s);
        }
        println!()
    }
}
