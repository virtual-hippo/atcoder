use proconio::input;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for i in 0..n {
        let l = *map.get(&(a[i] - 1)).unwrap_or(&0);
        *map.entry(a[i]).or_insert(0) = l + 1;
    }

    let ans = map.values().max().unwrap();
    println!("{}", ans);
}
