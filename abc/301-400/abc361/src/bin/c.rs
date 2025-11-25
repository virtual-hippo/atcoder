use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let a = a.iter().copied().sorted().collect_vec();
    let ans = (0..=k).map(|i| a[i + n - k - 1] - a[i]).min().unwrap();

    println!("{}", ans);
}
