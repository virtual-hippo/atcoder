use itertools::*;
use proconio::{fastout, input};
// 2025.12.28
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
    }
    let a = a.iter().copied().map(|x| x % m).collect_vec();

    let cnt_table = (0..l)
        .map(|i| {
            (0..m)
                .map(|j| (i..n).step_by(l).map(|di| (m + j - a[di]) % m).sum::<usize>() as u64)
                .collect_vec()
        })
        .collect_vec();

    let dp = (1..l).fold(cnt_table[0].clone(), |old, i| {
        iproduct!(0..m, 0..m).fold(vec![u64::MAX; m], |mut dp, (j, k)| {
            let nj = (j + k) % m;
            dp[nj] = dp[nj].min(old[j] + cnt_table[i][k]);
            dp
        })
    });

    let ans = dp[0];
    println!("{}", ans);
}
