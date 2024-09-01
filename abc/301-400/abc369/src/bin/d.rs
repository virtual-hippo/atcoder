use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut dp = vec![vec![0; 2]; n];

    dp[0][1] = a[0];
    for i in 1..n {
        for j in 0..2 {
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            if j % 2 == 1 {
                dp[i][(j + 1) % 2] = dp[i][(j + 1) % 2].max(dp[i - 1][j] + a[i] * 2);
            } else {
                dp[i][(j + 1) % 2] = dp[i][(j + 1) % 2].max(dp[i - 1][j] + a[i]);
            }
        }
    }

    let ans = dp[n - 1][0].max(dp[n - 1][1]);
    println!("{}", ans);
}
