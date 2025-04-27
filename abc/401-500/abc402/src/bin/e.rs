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
        x: usize,
        scp: [(f64,usize,f64); n],
    }

    // dp[state][i] = 残り i 円で、解いた集合が state の状態からの期待値の最大値
    let mut dp = vec![vec![0.0; x + 1]; 1 << n];

    for x in 0..x + 1 {
        for state in 0..(1 << n) {
            for i in 0..n {
                let si = scp[i].0;
                let ci = scp[i].1;
                if x < ci {
                    continue;
                }
                if (state >> i) & 1 == 1 {
                    continue;
                }

                let pi = scp[i].2 / 100.0;

                let val = pi * (dp[state | (1 << i)][x - ci] + si) + (1.0 - pi) * dp[state][x - ci];
                if val > dp[state][x] {
                    dp[state][x] = val;
                }
            }
        }
    }

    let ans = dp[0][x];
    println!("{}", ans);
}
