use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let n = s.len();
    let m = 32;

    let init = s
        .chars()
        .enumerate()
        .map(|(i, ch)| match ch {
            'L' => i - 1,
            'R' => i + 1,
            _ => unreachable!(),
        })
        .collect_vec();

    let dp = (1..m).fold(vec![init], |mut dp, _| {
        let pre = dp.len() - 1;
        let new = (0..n).map(|i| dp[pre][dp[pre][i]]).collect_vec();
        dp.push(new);
        dp
    });

    let k = 10 * n;

    let ans = (0..n)
        .map(|i| (0..m).rev().filter(|&j| k >> j & 1 == 1).fold(i, |now, d| dp[d][now]))
        .fold(vec![0; n], |mut acc, i| {
            acc[i] += 1;
            acc
        });
    print_vec_1line(&ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
