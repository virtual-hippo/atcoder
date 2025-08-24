#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        lr: [(usize, usize); m],
    }

    let mut g = vec![usize::MAX; 2 * n + 2];
    for i in 0..n {
        g[i] = i + 1;
        g[i + n + 1] = i + n + 1 + 1;
    }

    for (l, r) in lr {
        // let l = l - 1;
        // let r = r - 1;
        g.swap(l - 1, n + 1 + l - 1);
        g.swap(r, n + 1 + r);
    }

    let mut next = 0;
    for _ in 0..n {
        next = g[next];

        if next > n {
            print!("{}", t[next - n - 2]);
        } else {
            print!("{}", s[next - 1]);
        }
    }
}
