#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, i64};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    for i in 0..n {
        let mut max = (i, 0);
        for j in 0..n {
            if j == i {
                continue;
            }
            let d = calc_d(xy[i], xy[j]);
            if d > max.1 {
                max = (j, d);
            }
        }
        println!("{}", max.0 + 1);
    }
}

fn calc_d(pos1: (i64, i64), pos2: (i64, i64)) -> i64 {
    (pos2.0 - pos1.0).pow(2) + (pos2.1 - pos1.1).pow(2)
}
