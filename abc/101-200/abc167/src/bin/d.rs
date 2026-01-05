use itertools::*;
use proconio::{input, marker::*};

// ダブリング
fn main() {
    input! {
        n: usize,
        k: u64,
        a: [Usize1; n],
    }

    let m = 63;

    let init = vec![a];

    let dp = (1..m).fold(init, |mut dp, _| {
        let pre = dp.len() - 1;
        let new = (0..n).map(|i| dp[pre][dp[pre][i]]).collect_vec();
        dp.push(new);
        dp
    });

    let ans = (0..m).rev().filter(|&j| k >> j & 1 == 1).fold(0, |now, j| dp[j][now]) + 1;
    println!("{}", ans);
}
