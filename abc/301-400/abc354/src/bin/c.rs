#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet, VecDeque};
use superslice::Ext;

use std::cmp::{Ordering, Reverse};

#[fastout]
fn main() {
    input! {
        n: usize,
        ac: [(usize,usize); n],
    }

    let sorted = ac
        .iter()
        .copied()
        .enumerate()
        .sorted_by_key(|&(i, (a, c))| (Reverse(a), c, i))
        .collect_vec();

    let mut set = HashSet::new();
    let mut min_c = sorted[0].1 .1;

    for i in 0..n {
        let (j, (_, c)) = sorted[i];
        if min_c < c {
            set.insert(j);
        } else {
            min_c = c;
        }
    }

    let ans = (0..n).filter(|&i| !set.contains(&i)).collect_vec();
    println!("{}", ans.len());
    for i in ans {
        print!("{} ", i + 1);
    }
}
