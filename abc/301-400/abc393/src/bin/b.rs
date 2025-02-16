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
    for i in 0..s.len() {
        for j in 1..s.len() {
            if i + j + j >= s.len() {
                continue;
            }
            if s[i] == 'A' && s[i + j] == 'B' && s[i + j + j] == 'C' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
