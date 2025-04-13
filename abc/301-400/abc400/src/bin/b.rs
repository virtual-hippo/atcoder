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
        m: usize,
    }
    let mut x = 0;
    for i in 0..=m {
        let v = n.pow(i as u32);
        if v > 1_000_000_000 {
            println!("inf");
            return;
        }
        x += v;
        if x > 1_000_000_000 {
            println!("inf");
            return;
        }
    }

    println!("{}", x);
}
