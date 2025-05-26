// ランレングス圧縮

use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        k: usize,
        s: String
    }
    let run_length = s.chars().dedup_with_count().collect_vec();

    let ans = run_length
        .iter()
        .filter(|&&(_, ch)| ch == 'O')
        .map(|(cnt, _)| cnt / k)
        .sum::<usize>();

    println!("{}", ans);
}
