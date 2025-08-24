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
    }

    let mut map = FxHashMap::default();

    for _ in 0..n {
        input! {
            rc: (i64,i64),
        }
        *map.entry(rc).or_insert(0) += 1;
    }
    if map.len() == 1 {
        println!("0");
        return;
    }

    let r = map.keys().copied().map(|(r, _)| r).sorted().collect::<Vec<_>>();
    let c = map.keys().copied().map(|(_, c)| c).sorted().collect::<Vec<_>>();
    let d = (r[r.len() - 1] - r[0]).max(c[c.len() - 1] - c[0]);

    let ans = if d % 2 == 0 { d / 2 } else { d / 2 + 1 };
    println!("{}", ans);
}
