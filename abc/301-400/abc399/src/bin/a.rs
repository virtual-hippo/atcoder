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
        s: Chars,
        t: Chars,
    }
    let mut ans = 0;
    for i in 0..n {
        if s[i] != t[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
