use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            s: String,
        }
        *map.entry(s).or_insert(0) += 1;
    }
    let ans = map.iter().max_by_key(|&(_key, value)| value).unwrap().0;
    println!("{}", ans);
}
