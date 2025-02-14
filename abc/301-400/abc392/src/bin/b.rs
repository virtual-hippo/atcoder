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
        m: usize,
        a: [usize; m],
    }
    let mut set = FxHashSet::default();
    for i in 0..m {
        set.insert(a[i]);
    }

    let mut set2 = BTreeSet::new();
    for i in 1..=n {
        if !set.contains(&i) {
            set2.insert(i);
        }
    }

    println!("{} ", set2.len());
    for v in set2.iter() {
        print!("{} ", v);
    }
}
