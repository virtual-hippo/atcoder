use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
        cards: [[usize; 2]; n],
    }

    let mut dp = vec![vec![0,0]; n];
    dp[0] = vec![1,1];
    for i in 1..n {
        for pre in 0..2 {
            for nxt in 0..2 {
                if cards[i-1][pre] != cards[i][nxt] {
                    dp[i][nxt] += dp[i-1][pre];
                }
            }
        }
        dp[i][0] %= MOD;
        dp[i][1] %= MOD;
    }

    println!("{}", (dp[n-1][0] + dp[n-1][1]) % MOD);
}
