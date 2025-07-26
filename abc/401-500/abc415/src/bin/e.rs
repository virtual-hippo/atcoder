use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
        p: [i64; h+w-1],
    }

    let mut dp = vec![vec![i64::MAX; w]; h];
    dp[h - 1][w - 1] = (p[p.len() - 1] - a[h - 1][w - 1]).max(0);

    for (i, j) in iproduct!((0..h).rev(), (0..w).rev()) {
        if i > 0 {
            let cost = (dp[i][j] + p[i + j - 1] - a[i - 1][j]).max(0);
            if dp[i - 1][j] == i64::MAX {
                dp[i - 1][j] = cost;
            } else {
                dp[i - 1][j] = dp[i - 1][j].min(cost);
            }
        }
        if j > 0 {
            let cost = (dp[i][j] + p[i + j - 1] - a[i][j - 1]).max(0);
            if dp[i][j - 1] == i64::MAX {
                dp[i][j - 1] = cost;
            } else {
                dp[i][j - 1] = dp[i][j - 1].min(cost);
            }
        }
    }

    let ans = dp[0][0];
    println!("{}", ans);
}
