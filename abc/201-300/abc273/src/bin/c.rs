#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::{BTreeMap, VecDeque};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map = BTreeMap::new();
    for i in 0..n {
        *map.entry(Reverse(a[i])).or_insert(0) += 1;
    }

    map.iter().for_each(|(_, v)| println!("{}", v));

    (map.len()..n).for_each(|_| {
        println!("{}", 0);
    });
}
