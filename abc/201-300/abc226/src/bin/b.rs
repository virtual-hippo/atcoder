use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::with_capacity(n);
    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        }
        set.insert(a);
    }
    println!("{}", set.len());
}
