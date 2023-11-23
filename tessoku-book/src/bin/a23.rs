use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }
    let mut dp = vec![vec![usize::MAX - 1; 1 << n]; m + 1];
    dp[0][0] = 0;
    for i in 1..m + 1 {
        for j in 0..(1 << n) {
            let v = (0..n)
                .filter(|&k| (j & (1 << k) != 0) || a[i - 1][k] == 1)
                .fold(0, |sum, k| sum + (1 << k));
            dp[i][j] = dp[i][j].min(dp[i - 1][j]);
            dp[i][v] = dp[i][v].min(dp[i - 1][j] + 1);
        }
    }
    if dp[m][(1 << n) - 1] == usize::MAX - 1 {
        println!("-1");
    } else {
        println!("{}", dp[m][(1 << n) - 1]);
    }
}
