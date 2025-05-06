#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut b_set = BTreeSet::new();
    let mut a = vec![];

    for _ in 0..n {
        input! {
            aa: usize,
        }
        b_set.insert(aa);
        a.push(aa);
    }

    let mut b = vec![];
    for v in b_set.iter() {
        b.push(*v);
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        let res = b.upper_bound(&a[i]);
        ans[b.len() - res] += 1;
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}
