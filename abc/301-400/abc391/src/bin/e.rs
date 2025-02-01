#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn main() {
    input! {
        _n: usize,
        // 3_usize.pow(n as u32)
        a: Chars,
    }

    // ** 木DP **
    // dp[v][i] = v をi にするコスト (v を根とする部分木について)
    // i: 0 | 1
    // ボトムアップにやる

    let a: Vec<usize> = a.iter().map(|&ch| ((ch as u8) - b'0') as usize).collect();
    let mut dp = vec![vec![1_usize; 2]; a.len()];

    for i in 0..a.len() {
        dp[i][a[i]] = 0;
    }

    while dp.len() > 1 {
        let mut old = vec![vec![usize::MAX; 2]; dp.len() / 3];
        std::mem::swap(&mut dp, &mut old);

        for l in (0..old.len()).step_by(3) {
            for s in 0..8 {
                let cost: usize = (0..3).map(|i| old[l + i][(s >> i) & 1]).sum();

                // 多数決の結果
                let x = if s.count_ones() >= 2 { 1 } else { 0 };
                dp[l / 3][x] = dp[l / 3][x].min(cost);
            }
        }
    }

    let ans = dp[0][0].max(dp[0][1]);
    println!("{}", ans);
}
