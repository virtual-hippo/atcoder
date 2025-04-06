#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

pub fn binary_search<F: Fn(u64) -> bool>(initial_pos: (u64, u64), is_ok: F) -> (u64, u64) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while left + 1 < right {
        let mid = (left + right) / 2;
        if is_ok(mid) {
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
        n: u64,
    }

    let mut ans = 0;

    for a in 1..61 {
        let a2 = 1 << a;
        let is_ok = |b: u64| b * b <= n / a2;
        let (left, _) = binary_search((0, 1_000_000_001), is_ok);
        ans += (left + 1) / 2;
    }

    println!("{}", ans);
}
