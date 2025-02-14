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
        a: usize,
        b: usize,
        c: usize,
    }
    if a * b == c {
        println!("Yes");
    } else if a * c == b {
        println!("Yes");
    } else if b * c == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
