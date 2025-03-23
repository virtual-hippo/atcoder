#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map: HashMap<usize, usize> = HashMap::default();
    for &v in a.iter() {
        *map.entry(v).or_insert(0) += 1;
    }

    let mut ans = (0, -1);
    for i in 0..n {
        if let Some(&v) = map.get(&a[i]) {
            if v == 1 && a[i] >= ans.0 {
                ans = (a[i], i as i64 + 1);
            }
        }
    }

    println!("{}", ans.1);
}
