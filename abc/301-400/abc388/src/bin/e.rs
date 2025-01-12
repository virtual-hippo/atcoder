#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

pub fn binary_search<F: Fn(usize) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (i64, i64) {
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
    (left, right)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let is_ok = |k: usize| (0..k).filter(|&i| a[i] * 2 <= a[n - k + i]).count() == k;
    let ans = binary_search((0, n as i64 / 2 + 1), is_ok).0;
    println!("{}", ans);
}
