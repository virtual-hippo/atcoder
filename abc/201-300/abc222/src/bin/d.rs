use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    const MOD: usize = 998244353;
    let m = 3000;
    let mut dp = vec![vec![0; m + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n + 1 {
        for j in 0..m {
            dp[i][j + 1] += dp[i][j];
            dp[i][j + 1] %= MOD;
        }
        if i != n {
            for j in a[i]..=b[i] {
                dp[i + 1][j] += dp[i][j];
                dp[i + 1][j] %= MOD;
            }
        }
    }
    println!("{}", dp[n][m]);
}
