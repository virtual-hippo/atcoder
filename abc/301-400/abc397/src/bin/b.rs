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
        s: Chars,
    }
    let mut ans = 0;
    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            ans += 1;
        }
    }
    if s[0] == 'o' {
        ans += 1;
    }
    if s[s.len() - 1] == 'i' {
        ans += 1;
    }

    println!("{}", ans);
}
