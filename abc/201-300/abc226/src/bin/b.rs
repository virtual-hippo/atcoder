use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }
    let mut aa = HashSet::with_capacity(n);
    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        }
        aa.insert(a);
    }
    println!("{}", aa.len());
}
