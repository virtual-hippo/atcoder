#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

fn calc_d(pos1: (i64, i64), pos2: (i64, i64)) -> i64 {
    (pos2.0 - pos1.0).pow(2) + (pos2.1 - pos1.1).pow(2)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64,i64); n],
    }
    let mut ans = 0.0;
    let mut now = (0, 0);
    for i in 0..n {
        ans += (calc_d(now, xy[i]) as f64).sqrt();
        now = xy[i];
    }
    ans += (calc_d(now, (0, 0)) as f64).sqrt();
    println!("{}", ans);
}
