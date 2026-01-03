use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let xy = xy
        .iter()
        .copied()
        .sorted_by_key(|&(x, y)| (x, std::cmp::Reverse(y)))
        .collect_vec();

    let dp = (0..n).fold(vec![], |mut dp, i| {
        let (_, y) = xy[i];

        let pos = dp.partition_point(|&v| v < y);
        if pos == dp.len() {
            dp.push(y);
        } else {
            dp[pos] = y;
        }

        dp
    });
    println!("{}", dp.len());
}
