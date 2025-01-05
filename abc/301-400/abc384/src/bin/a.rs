#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        c1: char,
        c2: char,
        mut s: Chars,
    }

    for i in 0..n {
        if s[i] != c1 {
            s[i] = c2;
        }
        print!("{}", s[i]);
    }
}
