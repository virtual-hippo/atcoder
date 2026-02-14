use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        d: [usize; n],
    }

    let d = d.iter().copied().sorted().collect_vec();
    let ans: usize = (0..n - k).map(|i| d[i]).sum();

    println!("{}", ans);
}
