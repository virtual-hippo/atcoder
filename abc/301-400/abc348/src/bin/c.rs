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
    let mut map = FxHashMap::default();
    for _ in 0..n {
        input! {
            a: usize,
            c: usize,
        }
        if let Some(&v) = map.get(&c) {
            if v > a {
                map.insert(c, a);
            }
        } else {
            map.insert(c, a);
        }
    }
    let ans = map.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1;

    println!("{}", ans);
}
