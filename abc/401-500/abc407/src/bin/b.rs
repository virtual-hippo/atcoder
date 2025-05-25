use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let vals = iproduct!(1..7, 1..7)
        .map(|(i, j): (usize, usize)| (i + j, i.abs_diff(j)))
        .collect::<Vec<_>>();
    let oks = vals.iter().filter(|&&v| v.0 >= x || v.1 >= y).count();

    let ans = oks as f64 / vals.len() as f64;
    println!("{}", ans);
}
