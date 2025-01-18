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
        mut a: usize,
        mut b: usize,
        c: usize,
    }
    if a < c {
        a += 24;
    }
    if b < c {
        b += 24;
    }

    if c < a && a < b {
        println!("Yes");
    } else {
        println!("No");
    }
}
