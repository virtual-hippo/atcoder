use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;

    for i in 0..n - 1 {
        dp[i + 1] = dp[i + 1].min(dp[i] + h[i].abs_diff(h[i + 1]));

        if i + 2 < n {
            dp[i + 2] = dp[i + 2].min(dp[i] + h[i].abs_diff(h[i + 2]));
        }
    }

    let ans = dp[n - 1];
    println!("{}", ans);
}
