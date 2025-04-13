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
        k: usize,
    }
    let mut a = vec![];
    let mut s = vec![0_u64];

    for i in 0..n + 1 {
        if i < k {
            a.push(1);
            let next = s[i] + 1;
            s.push(next);
        } else {
            a.push((s[i] + 1000000000 - s[i - k]) % 1000000000);
            let next = (s[i] + a[i]) % 1000000000;
            s.push(next);
        }
    }

    let ans = a[n];
    println!("{}", ans);
}
