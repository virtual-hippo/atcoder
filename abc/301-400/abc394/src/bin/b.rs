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
        mut s: [String;n],
    }

    s.sort_by(|a, b| a.len().cmp(&b.len()));
    for str in s {
        print!("{}", str);
    }
}
