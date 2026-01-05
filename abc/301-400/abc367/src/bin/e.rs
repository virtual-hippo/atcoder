use itertools::*;
use proconio::{fastout, input, marker::*};

// ダブリング
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: [Usize1; n],
        a: [usize; n],
    }

    let m = 60;

    let dp = (1..m).fold(vec![x], |mut dp, _| {
        let pre = dp.len() - 1;
        let new = (0..n).map(|i| dp[pre][dp[pre][i]]).collect_vec();
        dp.push(new);
        dp
    });

    let ans = (0..n)
        .map(|i| (0..m).rev().filter(|&j| k >> j & 1 == 1).fold(i, |acc, j| dp[j][acc]))
        .map(|i| a[i])
        .collect_vec();

    print_vec_1line(&ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
