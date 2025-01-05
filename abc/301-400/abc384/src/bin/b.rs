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
        r: i64,
    }

    let mut now = r;
    for _ in 0..n {
        input! {
            d: usize,
            a: i64,
        }
        if d == 1 && 1600 <= now && now <= 2799 {
            now += a;
        }
        if d == 2 && 1200 <= now && now <= 2399 {
            now += a;
        }
    }

    println!("{}", now);
}
