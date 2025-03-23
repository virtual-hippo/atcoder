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
    }
    if n % 2 == 0 {
        for _ in 0..(n - 1) / 2 {
            print!("-");
        }
        print!("==");
        for _ in 0..(n - 1) / 2 {
            print!("-");
        }
    } else {
        for _ in 0..n / 2 {
            print!("-");
        }
        print!("=");
        for _ in 0..n / 2 {
            print!("-");
        }
    }
}
