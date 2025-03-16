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
        a: [usize; n],
    }
    let mut map = FxHashMap::default();
    for &x in &a {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut left = FxHashMap::default();

    let mut ans = 0;

    for i in 0..n - 1 {
        *left.entry(a[i]).or_insert(0) += 1;
        if let Some(x) = map.get_mut(&a[i]) {
            if *x == 1 {
                map.remove(&a[i]);
            } else {
                *x -= 1;
            }
        }
        ans = ans.max(map.len() + left.len());
    }

    println!("{}", ans);
}
