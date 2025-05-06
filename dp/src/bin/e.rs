use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ww: usize,
        wv: [(usize, usize); n],
    }

    let max_value = 100_100;
    let mut dp = vec![vec![usize::MAX; max_value]; n + 1];
    dp[0][0] = 0;

    for i in 1..n + 1 {
        let (w, v) = wv[i - 1];
        for j in 0..max_value {
            if dp[i - 1][j] != usize::MAX {
                dp[i][j] = dp[i][j].min(dp[i - 1][j]);
            }

            if j >= v && dp[i - 1][j - v] != usize::MAX {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - v] + w);
            }
        }
    }

    let mut ans = 0;
    for j in 0..max_value {
        if dp[n][j] <= ww {
            ans = ans.max(j);
        }
    }

    println!("{}", ans);
}
