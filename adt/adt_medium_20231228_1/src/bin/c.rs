use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; 4*n -1],
    }
    let mut map = HashMap::new();
    for i in 0..(4 * n - 1) {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let ans = map
        .iter()
        .filter(|&(_, v)| *v == 3)
        .map(|(k, _)| *k)
        .collect::<Vec<usize>>()[0];
    println!("{}", ans);
}
