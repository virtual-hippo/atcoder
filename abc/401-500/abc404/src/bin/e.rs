#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, usize};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let (c, a) = {
        let mut c = vec![0; n];
        for i in 1..n {
            input! {
                cc: usize,
            }
            c[i] = cc;
        }

        let mut a = vec![1; n];
        for i in 1..n {
            input! {
                aa: usize,
            }
            a[i] = aa;
        }
        (c, a)
    };

    let mut ans = 0;
    let mut nc = vec![];

    for i in 1..n {
        nc.push(c[i]);
        if a[i] > 0 {
            ans += solve(&nc);
            nc.clear();
        }
    }

    println!("{}", ans);
}

fn solve(nc: &Vec<usize>) -> usize {
    let n = nc.len();
    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;

    for i in 0..n {
        let mut now = usize::MAX;
        for j in 0..nc[i] {
            if i >= j {
                now = now.min(dp[i - j]);
            }
        }
        dp[i + 1] = now + 1;
    }

    dp[n]
}
