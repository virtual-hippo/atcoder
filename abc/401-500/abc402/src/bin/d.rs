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
        m: usize,
    }
    let edges = {
        let mut edges = vec![];
        for _ in 0..m {
            input! {
                a: usize,
                b: usize,
            }
            // let a = a - 1;
            // let b = b - 1;
            edges.push((a, b));
        }
        edges.sort();
        edges
    };

    let all = (m * (m - 1)) / 2;
    let mut cnt = vec![0; n * 2];

    for i in 0..m {
        let dist = (edges[i].0 + edges[i].1) % n;
        cnt[dist] += 1;
    }
    let ans = all - cnt.iter().filter(|&&v| v > 1).map(|v| (v * (v - 1)) / 2).sum::<usize>();
    println!("{}", ans);
}
