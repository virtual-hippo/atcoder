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
        x: i64,
        a: i64,
        d: i64,
        n: usize,
    }

    let l = a + d * 0;
    let r = a + d * (n as i64 - 1);
    let in_range = |v| l.min(r) <= v && v <= l.max(r);

    if !in_range(x) {
        let ans = ((x - l).abs()).min((x - r).abs());
        println!("{}", ans);
        return;
    }

    if d == 0 {
        let ans = (x - a).abs();
        println!("{}", ans);
        return;
    }

    if (x - a) % d == 0 {
        let ans = 0;
        println!("{}", ans);
    } else {
        let m = (x - a) % d;
        let ans = (m.abs()).min((d - m).abs());
        println!("{}", ans);
    }
}
