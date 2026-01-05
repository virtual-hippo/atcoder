use itertools::*;
use proconio::input;

pub fn digits_to_vec(x: usize) -> Vec<usize> {
    if x / 10 == 0 {
        vec![x % 10]
    } else {
        digits_to_vec(x / 10).into_iter().chain(std::iter::once(x % 10)).collect()
    }
}

fn f(x: usize) -> usize {
    x - digits_to_vec(x).iter().sum::<usize>()
}

// ダブリング
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let m = 30;
    let r = n + 1;

    let init = vec![(0..r).map(f).collect_vec()];

    let dp = (1..m).fold(init, |mut dp, _| {
        let pre = dp.len() - 1;
        let new = (0..r).map(|i| dp[pre][dp[pre][i]]).collect_vec();
        dp.push(new);
        dp
    });

    for i in 1..r {
        let ans = (0..m).rev().filter(|&d| k >> d & 1 == 1).fold(i, |now, d| dp[d][now]);
        println!("{}", ans);
    }
}
