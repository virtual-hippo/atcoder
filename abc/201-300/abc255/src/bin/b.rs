#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};

fn calc_d(pos1: (i64, i64), pos2: (i64, i64)) -> i64 {
    (pos2.0 - pos1.0).pow(2) + (pos2.1 - pos1.1).pow(2)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(i64, i64); n],
    }

    let mut mx = 0;

    for i in 0..n {
        let mut mn = i64::MAX;
        for j in 0..k {
            let p = a[j] - 1;
            mn = mn.min(calc_d(xy[i], xy[p]));
        }
        mx = mx.max(mn);
    }

    let ans = (mx as f64).sqrt();
    println!("{}", ans);
}
