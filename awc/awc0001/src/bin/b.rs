use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        p: [usize; n],
    }
    let p = p
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, x)| l <= x && x <= r)
        .sorted_by_key(|&(i, x)| (std::cmp::Reverse(x), i))
        .collect_vec();
    let ans = if p.is_empty() {
        format!("{}", -1)
    } else {
        format!("{}", p[0].0 + 1)
    };

    println!("{}", ans);
}
