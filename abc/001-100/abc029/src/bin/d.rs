use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    let mut dp = [[[0; 11]; 2]; 11];

    dp[0][0][0] = 1;

    for i in 0..n.len() {
        for j in 0..10 {
            dp[i + 1][1][j] += dp[i][1][j] * 9;
            dp[i + 1][1][j + 1] += dp[i][1][j];

            let ni = n[i] as usize - '0' as usize;

            if ni > 1 {
                dp[i + 1][1][j] += dp[i][0][j] * (ni - 1);
                dp[i + 1][1][j + 1] += dp[i][0][j];
            } else if ni == 1 {
                dp[i + 1][1][j] += dp[i][0][j];
            }

            if ni == 1 {
                dp[i + 1][0][j + 1] = dp[i][0][j];
            } else {
                dp[i + 1][0][j] = dp[i][0][j];
            }
        }
    }

    let ans = (0..10).map(|j| (dp[n.len()][1][j] + dp[n.len()][0][j]) * j).sum::<usize>();
    println!("{}", ans);
}
