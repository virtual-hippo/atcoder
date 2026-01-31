use itertools::*;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let s = {
        let mut s = vec![vec![]; n];
        for i in 0..n {
            input! {
                x: Chars,
            }
            s[n - i - 1] = x;
        }
        s
    };

    let cost_r = (0..n)
        .map(|i| {
            (0..n).fold(vec![0], |mut acc, j| {
                let tail = acc.len() - 1;
                let v = acc[tail] + if s[i][j] == '#' { 1 } else { 0 };
                acc.push(v);
                acc
            })
        })
        .collect_vec();

    let cost_c = (0..n).fold(vec![vec![0; n]], |mut acc, i| {
        let new_row = (0..n).map(|j| acc[i][j] + if s[i][j] == '.' { 1 } else { 0 }).collect_vec();
        acc.push(new_row);
        acc
    });

    let mut init = vec![usize::MAX; n + 1];
    init[0] = 0;
    let ans = (0..n + 1).fold(init, |old, i| {
        let dp = (0..n).fold(old, |mut dp, j| {
            dp[j + 1] = dp[j + 1].min(dp[j].saturating_add(cost_c[i][j]));
            dp
        });

        if i < n {
            (0..n + 1).map(|j| dp[j].saturating_add(cost_r[i][j])).collect()
        } else {
            dp
        }
    })[n];

    println!("{}", ans);
}
