use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        pab: [(usize,usize,usize); n],
        q: usize,
    }
    let m = 1_000;

    let dp = {
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 0..m + 1 {
            dp[n][i] = i;
        }

        for i in (0..n).rev() {
            let (p, a, b) = pab[i];

            for j in 0..m + 1 {
                let nj = if j <= p { j + a } else { j.saturating_sub(b) };
                dp[i][j] = dp[i + 1][nj];
            }
        }

        dp
    };

    let bs = std::iter::once(0_usize)
        .chain(pab.iter().scan(0_usize, |acc, &(_, _, x)| {
            *acc += x;
            Some(*acc)
        }))
        .collect::<Vec<_>>();

    for _ in 0..q {
        input! {
            x: usize,
        }

        let ans = if x > m {
            let i = bs.upper_bound(&(x - m - 1));

            if i < n {
                dp[i][x - bs[i]]
            } else {
                x - bs[n]
            }
        } else {
            dp[0][x]
        };

        println!("{}", ans);
    }
}
