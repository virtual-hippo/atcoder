// 2025.03.20 解説AC
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
        t: [usize; n],
    }
    let mut dp = vec![ModInt998244353::new(0); x + 1];
    dp[0] = ModInt998244353::new(1);

    let p: ModInt998244353 = ModInt998244353::new(1) / n;
    // 以下のようにも書ける
    // let p: ModInt998244353 = ModInt998244353::new(n).inv();

    for i in 0..x + 1 {
        for j in 0..n {
            let ni = i + t[j];
            if ni <= x {
                let added = dp[i] * p;
                dp[ni] += added;
            }
        }
    }

    let ans = (0..x + 1)
        .filter(|i| i + t[0] > x)
        .map(|i| dp[i] * p)
        .fold(ModInt998244353::new(0), |acc, v| acc + v)
        .val();
    println!("{}", ans);
}
