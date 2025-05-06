use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n],
    }
    let mut dp = vec![[0; 3]; n + 1];
    for i in 0..n {
        let (a, b, c) = abc[i];
        dp[i + 1][0] = (dp[i][1] + a).max(dp[i][2] + a);
        dp[i + 1][1] = (dp[i][0] + b).max(dp[i][2] + b);
        dp[i + 1][2] = (dp[i][0] + c).max(dp[i][1] + c);
    }

    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
