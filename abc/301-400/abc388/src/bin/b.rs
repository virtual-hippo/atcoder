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
        d: usize,
        tl: [(usize, usize); n],
    }

    for k in 0..d {
        let k = k + 1;
        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(tl[i].0 * (tl[i].1 + k));
        }
        println!("{}", ans);
    }
}
