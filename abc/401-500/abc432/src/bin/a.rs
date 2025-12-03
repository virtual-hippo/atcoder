use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let ans = vec![a, b, c]
        .iter()
        .copied()
        .sorted_by_key(|v| std::cmp::Reverse(*v))
        .collect_vec();
    println!("{}", ans[0] * 100 + ans[1] * 10 + ans[2]);
}
