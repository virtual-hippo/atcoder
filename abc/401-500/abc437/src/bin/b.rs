#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[usize; w]; h],
        b: [usize; n],
    }

    let b = b.iter().copied().collect::<HashSet<usize>>();

    let ans = a
        .iter()
        .map(|xs| xs.iter().filter(|&x| b.contains(x)).count())
        .max()
        .unwrap_or(0);

    println!("{}", ans);
}
