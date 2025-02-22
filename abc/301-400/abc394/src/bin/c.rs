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
        mut s: Chars,
    }
    for i in (1..s.len()).rev() {
        if s[i - 1] == 'W' && s[i] == 'A' {
            s[i - 1] = 'A';
            s[i] = 'C';
        }
    }

    let ans = s.iter().join("");
    println!("{}", ans);
}
