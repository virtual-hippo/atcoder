use itertools::*;
use proconio::{fastout, input};
use superslice::Ext;

// 累積和
fn generate_cumulative_sum<T>(xs: &[T]) -> Vec<T>
where
    T: Copy + std::ops::AddAssign + From<usize>,
{
    std::iter::once(T::from(0))
        .chain(xs.iter().scan(T::from(0), |acc, &x| {
            *acc += x;
            Some(*acc)
        }))
        .collect::<Vec<T>>()
}

#[fastout]
fn main() {
    input! {
        n: usize,
        pab: [(usize,usize,usize); n],
        q: usize,
    }

    const M: usize = 1_000;
    let dp = {
        let mut dp = vec![vec![0; M + 1]; n + 1];
        for i in 0..M + 1 {
            dp[n][i] = i;
        }

        for i in (0..n).rev() {
            let (p, a, b) = pab[i];
            for j in 0..M + 1 {
                let nj = if j <= p { j + a } else { j.saturating_sub(b) };
                dp[i][j] = dp[i + 1][nj];
            }
        }

        dp
    };

    let b = pab.iter().map(|&(_, _, b)| b).collect_vec();
    let bs = generate_cumulative_sum(&b);

    for _ in 0..q {
        input! {
            x: usize,
        }

        let ans = if x <= M {
            dp[0][x]
        } else {
            // x <= M まで -b し続ける
            let i = bs.lower_bound(&(x - M));

            if i < n {
                let x = x - bs[i];
                dp[i][x]
            } else {
                x - bs[n]
            }
        };
        println!("{}", ans);
    }
}
