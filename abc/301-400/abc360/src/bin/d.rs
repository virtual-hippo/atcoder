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
        t: i64,
        s: Chars,
        x: [i64; n],
    }

    let ants0 = (0..n).filter(|&i| s[i] == '0').map(|i| x[i] - t).sorted().collect::<Vec<i64>>();
    let ants1 = (0..n).filter(|&i| s[i] == '1').map(|i| x[i] + t).sorted().collect::<Vec<i64>>();
    let ants1_before = (0..n).filter(|&i| s[i] == '1').map(|i| x[i]).sorted().collect::<Vec<i64>>();

    let mut ans = 0;

    for v in ants0.iter() {
        let right = ants1.len() - ants1.lower_bound(v);
        let yobun = ants1_before.len() - ants1_before.lower_bound(&(*v + t));

        ans += right - yobun;
    }
    println!("{}", ans);
}
