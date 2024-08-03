use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    if s.len() % 2 == 1 {
        println!("0");
        return;
    }

    let n = s.len();
    let m = 998244353;

    let mut dp = vec![vec![0; 3001]; n + 1];
    dp[0][0] = 1;

    for i in 1..n + 1 {
        for j in 0..3000 {
            if s[i - 1] == '(' {
                if dp[i - 1][j] > 0 {
                    dp[i][j + 1] += dp[i - 1][j];
                    dp[i][j + 1] %= m;
                }
            }
            if s[i - 1] == ')' {
                if dp[i - 1][j] > 0 && j > 0 {
                    dp[i][j - 1] += dp[i - 1][j];
                    dp[i][j - 1] %= m;
                }
            }
            if s[i - 1] == '?' {
                if dp[i - 1][j] > 0 {
                    dp[i][j + 1] += dp[i - 1][j];
                    dp[i][j + 1] %= m;
                    if j > 0 {
                        dp[i][j - 1] += dp[i - 1][j];
                        dp[i][j - 1] %= m;
                    }
                }
            }
        }
    }

    let ans = dp[n][0];
    println!("{}", ans);
}
