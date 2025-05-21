use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![vec![usize::MAX; d]; k + 1]; n + 1];

    dp[0][0][0] = 0;

    for i in 0..n {
        for j in 0..=k {
            for kk in 0..d {
                if dp[i][j][kk] == usize::MAX {
                    continue;
                }

                if dp[i + 1][j][kk] == usize::MAX {
                    dp[i + 1][j][kk] = dp[i][j][kk];
                } else {
                    dp[i + 1][j][kk] = dp[i + 1][j][kk].max(dp[i][j][kk]);
                }

                if j == k {
                    continue;
                }

                let nv = dp[i][j][kk] + a[i];
                let nk = (nv) % d;
                if dp[i + 1][j + 1][nk] == usize::MAX {
                    dp[i + 1][j + 1][nk] = nv;
                } else {
                    dp[i + 1][j + 1][nk] = dp[i + 1][j + 1][nk].max(nv);
                }
            }
        }
    }

    let ans = dp[n][k][0];

    if ans != usize::MAX {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
