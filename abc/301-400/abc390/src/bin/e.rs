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
        x: usize,
    }

    let foods = {
        let mut ret = vec![vec![]; 3];
        for _ in 0..n {
            input! {
                v: usize,
                a: i64,
                c: usize,
            }
            let v = v - 1;
            ret[v].push((a, c));
        }

        ret
    };

    // d[v][i]: 接種カロリーが i の時のビタミン v の摂取量の最大値
    let d = (0..3)
        .map(|v| {
            let mut dp = vec![0; x + 1];
            for &(a, c) in foods[v].iter() {
                for i in (c..x + 1).rev() {
                    dp[i] = dp[i].max(dp[i - c] + a);
                }
            }
            dp
        })
        .collect_vec();

    let f = move |r: i64| {
        let total_opt = (0..3)
            .map(|v| {
                if d[v][x] < r {
                    return None;
                }
                let need_cal = d[v].lower_bound(&r);
                if !(need_cal < d[v].len() && d[v][need_cal] >= r) {
                    return None;
                }
                Some(need_cal)
            })
            .try_fold(0, |acc, x| x.map(|v| acc + v));

        match total_opt {
            Some(total) => total <= x,
            None => false,
        }
    };

    let ans = binary_search((0, 1_000_000_100), f).0;

    println!("{}", ans);
}

pub fn binary_search<F: Fn(i64) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (i64, i64) {
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
