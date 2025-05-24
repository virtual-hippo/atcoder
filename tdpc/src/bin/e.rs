use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: Chars,
    }

    let m = 1_000_000_007;

    let mut dp = [[[0; 102]; 2]; 10002];

    dp[0][0][0] = 1;

    for i in 0..n.len() {
        for j in 0..d {
            for k in 0..10 {
                dp[i + 1][1][(j + k) % d] += dp[i][1][j];
                dp[i + 1][1][(j + k) % d] %= m;
            }

            let ni = (n[i] as u8 - b'0') as usize;
            for k in 0..ni {
                dp[i + 1][1][(j + k) % d] += dp[i][0][j];
                dp[i + 1][1][(j + k) % d] %= m;
            }
            dp[i + 1][0][(j + ni) % d] = dp[i][0][j];
        }
    }

    let ans = dp[n.len()][1][0] + dp[n.len()][0][0] - 1;
    println!("{}", ans);
}
