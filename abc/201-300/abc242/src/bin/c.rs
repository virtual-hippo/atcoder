use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    const MOD: usize = 998_244_353;

    let mut dp = vec![vec![0; 10]; n+1];
    dp[1] = vec![0,1,1,1,1,1,1,1,1,1];
    for i in 1..n {
        for j in 1..10 {
            if j == 1 {
                dp[i+1][1] += dp[i][j];
                dp[i+1][2] += dp[i][j];
            } else if j == 9 {
                dp[i+1][8] += dp[i][j];
                dp[i+1][9] += dp[i][j];
            } else {
                dp[i+1][j-1] += dp[i][j];
                dp[i+1][j] += dp[i][j];
                dp[i+1][j+1] += dp[i][j];
            }
            dp[i+1][j] %= MOD;
        }
    }
    let ans = dp[n].iter().fold(0, |sum, &v| sum + v);
    println!("{}", ans % MOD);
}

