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
        s: String,
    }

    let ans = s.chars().filter(|&c| c.is_uppercase()).collect::<String>();
    println!("{}", ans);
}
