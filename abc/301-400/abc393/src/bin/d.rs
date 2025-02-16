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
    }
    let cnt1 = s.iter().filter(|&c| *c == '1').count();
    let mut now = 0;
    let mut ans = 0;

    for i in 0..n {
        if s[i] == '0' {
            ans += now.min(cnt1 - now);
        } else {
            now += 1;
        }
    }

    println!("{}", ans);
}
