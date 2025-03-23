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
    }
    let pow = |v: usize, p| {
        let mut ret = v;
        for _ in 0..p - 1 {
            ret *= v;
        }
        ret
    };

    let ans = pow(a, b) + pow(b, a);
    println!("{}", ans);
}
