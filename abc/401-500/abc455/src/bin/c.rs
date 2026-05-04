use itertools::*;
use proconio::input;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let s: usize = a.iter().copied().sum();

    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += a[i];
    }

    let vals = map.values().copied().sorted_by_key(|&x| std::cmp::Reverse(x)).collect_vec();
    let ans = s - (0..k.min(vals.len())).map(|i| vals[i]).sum::<usize>();
    println!("{}", ans);
}
