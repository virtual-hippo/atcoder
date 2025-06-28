use itertools::*;
use proconio::{fastout, input};
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let sorted = a.iter().enumerate().sorted_by_key(|&(_, &v)| Reverse(v)).collect::<Vec<_>>();

    let ans = sorted[1].0 + 1;
    println!("{}", ans);
}
