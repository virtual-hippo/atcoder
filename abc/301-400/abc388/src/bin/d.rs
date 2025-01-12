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
        mut a: [i64; n],
    }

    let mut d = vec![0; n + 1];
    for i in 0..n {
        a[i] += i as i64;
        a[i] -= (n - 1 - i) as i64;
        a[i] += d[i];

        if a[i] < 0 {
            d[n - ((-a[i]) as usize)] -= 1;
        }
        d[i + 1] += d[i];
    }

    for i in 0..n {
        if a[i] < 0 {
            print!("{} ", 0);
        } else {
            print!("{} ", a[i]);
        }
    }
}
