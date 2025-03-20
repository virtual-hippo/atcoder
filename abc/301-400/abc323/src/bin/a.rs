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

    for i in (0..16).filter(|&i| i % 2 == 1) {
        if s[i] != '0' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
