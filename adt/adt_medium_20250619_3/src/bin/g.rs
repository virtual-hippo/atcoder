use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut b_map = BTreeMap::new();

    for _ in 0..n {
        input! {
            s: usize,
            c: usize,
        }

        let mut s = s;
        let mut x = 1;
        while s % 2 == 0 {
            s /= 2;
            x *= 2;
        }
        *b_map.entry(s).or_insert(0) += c * x;
    }

    let ans: u64 = b_map.iter().map(|(_, v)| v.count_ones() as u64).sum();
    println!("{}", ans);
}
