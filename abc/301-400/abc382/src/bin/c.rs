#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64;n],
        b: [i64;m],
    }

    for i in 0..n - 1 {
        a[i + 1] = a[i].min(a[i + 1]);
    }

    for i in 0..m {
        let is_ok = |ai: usize| a[ai] > b[i];
        let (_, ans) = binary_search((-1, n as i64), is_ok);
        if ans == n {
            println!("{}", -1);
        } else {
            println!("{}", ans + 1);
        }
    }
}

pub fn binary_search<F: Fn(usize) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (usize, usize) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(mid as usize) {
            left = mid;
        } else {
            right = mid;
        }
    }
    (left as usize, right as usize)
}
