#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use superslice::Ext;

fn solve(_: usize) {
    input! {
        n: usize,
        wp: [(i64,i64); n],
    }
    let mut wp = wp
        .iter()
        .copied()
        .sorted_by_key(|&(w, p)| std::cmp::Reverse(w + p))
        .collect_vec();

    let mut powers: i64 = wp.iter().copied().map(|(_, p)| p).sum();
    let mut wehigts = 0;
    let mut ans = 0;

    while !wp.is_empty() {
        let (w, p) = wp.pop().unwrap();
        wehigts += w;
        powers -= p;

        if powers >= wehigts {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans);
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    (0..t).for_each(solve);
}
