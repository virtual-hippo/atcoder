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
        a: [u64; n],
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut ans = 0;
    for i in 0..n {
        let v = a[i] * (s[n] - s[i + 1]);
        ans += v;
    }

    println!("{}", ans);
}
