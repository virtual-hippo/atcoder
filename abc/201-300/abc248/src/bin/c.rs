use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    const MOD: usize = 998244353;

    // dp[i][j] 数列の先頭から i 項まで決めた際に、総和が j であるような数列の決め方の総数
    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..k {
            for l in 1..m + 1 {
                if j + l <= k {
                    dp[i + 1][j + l] += dp[i][j];
                    dp[i + 1][j + l] %= MOD;
                }
            }
        }
    }
    let ans = (1..k + 1).fold(0, |sum, i| (sum + dp[n][i]) % MOD);
    println!("{}", ans);
}
