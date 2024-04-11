use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: String,
        n: usize,
    }
    let mut dp = vec![vec![usize::MAX; t.len() + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..n + 1 {
        input! {
            a: usize,
            s: [String; a],
        }
        for j in 0..t.len() + 1 {
            if dp[i - 1][j] == usize::MAX {
                continue;
            }
            dp[i][j] = dp[i][j].min(dp[i - 1][j]);
            let start = j;
            for k in 0..a {
                if start + s[k].len() > t.len() {
                    continue;
                }
                if &t[start..start + s[k].len()] == s[k].as_str() {
                    dp[i][j + s[k].len()] = (dp[i - 1][j] + 1).min(dp[i][j + s[k].len()]);
                }
            }
        }
    }

    let ans = if dp[n][t.len()] == usize::MAX {
        -1
    } else {
        dp[n][t.len()] as i64
    };
    println!("{}", ans);
}
