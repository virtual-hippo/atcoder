#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: u128,
        p: [u128; n],
    }
    let is_ok = |x: u128| {
        let val = (0..n)
            .map(|i| {
                // (2k - 1) * p_i < x  <-->  (2k - 1) * p_i <= x-1
                // を満たす最大の k を求める
                let k = ((x - 1) / p[i] + 1) / 2;
                k * k * p[i]
            })
            .sum::<u128>();

        val <= m
    };

    let x = binary_search((0, m + 1), is_ok).0 as usize;

    let ans = {
        let (sum_k, sum_p) = (0..n)
            .map(|i| {
                let k = (((x - 1) / (p[i] as usize) + 1) / 2) as u128;
                (k, k * k * p[i])
            })
            .fold((0, 0), |(sum_k, sum_p), (dk, dp)| (dk + sum_k, dp + sum_p));
        sum_k + (m - sum_p) / (x as u128)
    };

    println!("{}", ans);
}

pub fn binary_search<F: Fn(u128) -> bool>(initial_pos: (u128, u128), is_ok: F) -> (u128, u128) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    (left, right)
}
