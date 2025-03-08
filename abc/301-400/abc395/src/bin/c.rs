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
    let mut ans = usize::MAX;
    let mut map = FxHashMap::default();
    for i in 0..n {
        if let Some(&j) = map.get(&a[i]) {
            ans = ans.min(i - j + 1);
        }
        map.insert(a[i], i);
    }
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
