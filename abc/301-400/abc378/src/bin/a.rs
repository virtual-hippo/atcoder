use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    let mut map = HashMap::new();
    for _ in 0..4 {
        input! {
            a: usize,
        }
        *map.entry(a).or_insert(0) += 1;
    }
    let ans: usize = map.iter().map(|(_, &v)| v / 2).sum();
    println!("{}", ans);
}
