use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        _n: usize,
        s: String,
    }

    let run_length = s.chars().map(|ch| ch as u8 - b'0').dedup_with_count().collect_vec();
    let mx_zero = run_length
        .iter()
        .filter(|&(_, v)| *v == 0)
        .map(|&(cnt, _)| cnt)
        .max()
        .unwrap_or(0);
    let mx_one = run_length
        .iter()
        .filter(|&(_, v)| *v == 1)
        .map(|&(cnt, _)| cnt)
        .max()
        .unwrap_or(0);

    let ans0 = run_length
        .iter()
        .fold(0, |acc, &(cnt, v)| acc + if v == 0 { cnt * 2 } else { cnt })
        - mx_zero * 2;
    let ans1 = run_length
        .iter()
        .fold(0, |acc, &(cnt, v)| acc + if v == 1 { cnt * 2 } else { cnt })
        - mx_one * 2;
    println!("{}", ans0.min(ans1));
}
