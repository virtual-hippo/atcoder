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
        k: usize,
        a: [u128; n],
    }

    let v = 10_u128.pow(k as u32);

    let mut now: u128 = 1;
    for i in 0..n {
        now = now.saturating_mul(a[i]);

        if now >= v || now == u128::MAX {
            now = 1;
        }
    }

    let ans = now;
    println!("{}", ans);
}
