use proconio::input;
use proconio::marker::Chars;

const OFF: usize = 0;
const ON: usize = 1;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
        s: Chars,
    }
    let mut dp = vec![[0, 0]; s.len()];
    if s[0] == 'a' {
        dp[0][OFF] = x;
        dp[0][ON] = z + y;
    } else {
        dp[0][OFF] = y;
        dp[0][ON] = z + x;
    }

    for i in 1..dp.len() {
        if s[i] == 'a' {
            dp[i][OFF] = std::cmp::min(dp[i-1][OFF] + x, dp[i-1][ON] + z + x);
            dp[i][ON] = std::cmp::min(dp[i-1][ON] + y, dp[i-1][OFF] + z + y);
        } else {
            dp[i][OFF] = std::cmp::min(dp[i-1][OFF] + y, dp[i-1][ON] + z + y);
            dp[i][ON] = std::cmp::min(dp[i-1][ON] + x, dp[i-1][OFF] + z + x);
        }
    }
    println!("{}", std::cmp::min(dp[s.len()-1][OFF], dp[s.len()-1][ON]));
}
