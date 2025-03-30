// 解説AC (よくわかっていない)
// 場合の数として考えるということはわかった
#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn comb(n: usize, k: usize) -> usize {
    let x = ((n - k + 1)..n + 1).fold(1, |acc, v| v * acc);
    let y = (1..k + 1).fold(1, |acc, v| v * acc);
    x / y
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = ModInt998244353::new(0);
    let mut dp = vec![ModInt998244353::new(0); k + 1];
    for i in 0..n {
        dp[0] += 1;
        let old = {
            let mut old: Vec<ModInt998244353> = vec![ModInt998244353::new(0); k + 1];
            std::mem::swap(&mut old, &mut dp);
            old
        };

        // a[i] に対して何個選ぶか考える
        for j in 0..k + 1 {
            for c in 0..k + 1 - j {
                // old[j]: a[0] から a[i-1] まで, j 個選ぶ場合の数
                // a[i].pow(c): a[i] から c 個選ぶ場合の数
                // comb(k - j, c): c 個の選び方
                dp[j + c] += old[j] * ModInt998244353::new(a[i]).pow(c as u64) * comb(k - j, c);
            }
        }

        // dp[k]: a[0] から a[i] まで, k 個選ぶ場合の数
        ans += dp[k];
    }

    println!("{}", ans);
}
