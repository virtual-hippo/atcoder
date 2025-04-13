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
    }
    if 400 % a == 0 {
        println!("{}", 400 / a);
    } else {
        println!("{}", -1);
    }
}
