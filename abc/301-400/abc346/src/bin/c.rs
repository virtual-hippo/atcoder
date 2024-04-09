use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(a[i]);
    }
    let sub: usize = set.iter().filter(|&&v| v <= k).sum();
    let ans = ((1 + k) * k / 2) - sub;
    println!("{}", ans);
}
