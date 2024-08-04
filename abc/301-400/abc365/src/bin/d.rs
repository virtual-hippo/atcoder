use proconio::{fastout, input, marker::Chars};

const R: usize = 0;
const S: usize = 1;
const P: usize = 2;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut dp = vec![vec![-1; 3]; n];

    if s[0] == 'R' {
        dp[0][R] = 0;
        dp[0][P] = 1;
    } else if s[0] == 'S' {
        dp[0][S] = 0;
        dp[0][R] = 1;
    } else if s[0] == 'P' {
        dp[0][P] = 0;
        dp[0][S] = 1;
    }
    for i in 0..(n - 1) {
        for j in [R, S, P] {
            if dp[i][j] == -1 {
                continue;
            }
            if j == R {
                if s[i + 1] == 'R' {
                    dp[i + 1][P] = dp[i + 1][P].max(dp[i][j] + 1);
                } else if s[i + 1] == 'S' {
                    dp[i + 1][S] = dp[i + 1][S].max(dp[i][j]);
                } else if s[i + 1] == 'P' {
                    dp[i + 1][P] = dp[i + 1][P].max(dp[i][j]);
                    dp[i + 1][S] = dp[i + 1][S].max(dp[i][j] + 1);
                }
            }
            if j == S {
                if s[i + 1] == 'R' {
                    dp[i + 1][R] = dp[i + 1][R].max(dp[i][j]);
                    dp[i + 1][P] = dp[i + 1][P].max(dp[i][j] + 1);
                } else if s[i + 1] == 'S' {
                    dp[i + 1][R] = dp[i + 1][R].max(dp[i][j] + 1);
                } else if s[i + 1] == 'P' {
                    dp[i + 1][P] = dp[i + 1][P].max(dp[i][j]);
                }
            }
            if j == P {
                if s[i + 1] == 'R' {
                    dp[i + 1][R] = dp[i + 1][R].max(dp[i][j]);
                } else if s[i + 1] == 'S' {
                    dp[i + 1][R] = dp[i + 1][R].max(dp[i][j] + 1);
                    dp[i + 1][S] = dp[i + 1][S].max(dp[i][j]);
                } else if s[i + 1] == 'P' {
                    dp[i + 1][S] = dp[i + 1][S].max(dp[i][j] + 1);
                }
            }
        }
    }
    let ans = dp[n - 1].iter().max().unwrap();
    println!("{}", ans);
}
