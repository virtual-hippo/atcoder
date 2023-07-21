use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: i64,
        a: [i64; n],
    }
    let mut dp = vec![vec![vec![-1; d as usize]; k+1]; n+1];
    dp[0][0][0] = 0;

    for ii in 0..n {
        for jj in 0..k+1 {
            for kk in 0..(d as usize) {
                if dp[ii][jj][kk] == -1 {
                    continue;
                }
                dp[ii+1][jj][kk] = dp[ii+1][jj][kk].max(dp[ii][jj][kk]);

                if jj != k {
                    dp[ii+1][jj+1][((kk as i64 + a[ii]) % d) as usize] = dp[ii+1][jj+1][((kk as i64 + a[ii]) % d) as usize].max(dp[ii][jj][kk] + a[ii]);
                }
            }
        }
    }
    println!("{}", dp[n][k][0]);
}

