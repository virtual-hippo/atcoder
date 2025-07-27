#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeMap, HashMap, HashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let a = a
        .iter()
        .map(|&x| x % m)
        .sorted_by_key(|v| std::cmp::Reverse(*v))
        .collect::<Vec<_>>();
    let b = b.iter().map(|&x| x % m).sorted().collect::<Vec<_>>();

    let mut c = 0;
    let mut idx = 0;

    for v in a.iter() {
        while idx < n && b[idx] + *v < m {
            idx += 1;
        }
        if idx >= n {
            break;
        }
        c += 1;
        idx += 1;
    }

    let ans = a.iter().sum::<usize>() + b.iter().sum::<usize>() - c * m;
    println!("{}", ans);
}
