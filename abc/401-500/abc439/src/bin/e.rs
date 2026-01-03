use itertools::*;
use proconio::{fastout, input};

// LIS
#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    let ab = ab
        .iter()
        .copied()
        .sorted_by_key(|&(a, b)| (a, std::cmp::Reverse(b)))
        .collect_vec();

    let dp = (0..n).fold(vec![], |mut dp, i| {
        let (_, b) = ab[i];
        let i = dp.partition_point(|&v| v < b);

        if i == dp.len() {
            dp.push(b);
        } else {
            dp[i] = b;
        }
        dp
    });

    let ans = dp.len();
    println!("{}", ans);
}
