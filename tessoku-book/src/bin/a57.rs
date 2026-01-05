use itertools::*;
use proconio::{input, marker::*};

// ダブリング

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
    }

    let dp = (1..32).fold(vec![a.clone()], |mut dp, _| {
        let pre = dp.len() - 1;
        let new = (0..n).map(|i| dp[pre][dp[pre][i]]).collect_vec();
        dp.push(new);
        dp
    });

    for _ in 0..q {
        input! {
            x: Usize1,
            y: usize,
        }

        let ans = (0..30).rev().filter(|&j| (y / (1 << j)) % 2 != 0).fold(x, |now, d| dp[d][now]) + 1;

        println!("{}", ans);
    }
}
