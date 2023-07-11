use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::with_capacity(n);
    for _ in 0..n {
        input! {
            mut s: Chars,
        }
        s.sort();
        *map.entry(s).or_insert(0) += 1;
    }
    let ans: u64 = map
        .iter()
        .filter(|&(_, &v)| v > 1)
        .fold(0, |sum, (_, &v)| sum + (v * (v - 1)) / 2);
    println!("{}", ans);
}
