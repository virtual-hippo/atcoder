use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ww: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; ww + 1]; n + 1];

    for i in 1..n + 1 {
        let (w, v) = wv[i - 1];
        for j in 0..ww + 1 {
            if j >= w {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - w] + v);
            }
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
        }
    }

    let ans = dp[n][ww];
    println!("{}", ans);
}
