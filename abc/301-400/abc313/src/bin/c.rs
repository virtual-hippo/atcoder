use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a = a.iter().copied().sorted().collect_vec();
    let s: usize = a.iter().copied().sum();
    let k = s % n;
    let avg = s / n;

    let mut b = vec![avg; n];
    for i in n - k..n {
        b[i] += 1;
    }

    let ans = (0..n).map(|i| b[i].abs_diff(a[i])).sum::<usize>() / 2;

    println!("{}", ans);
}
