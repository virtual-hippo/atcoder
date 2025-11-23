use itertools::*;
use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut map = HashMap::new();

    for i in 0..n {
        if i + k > n {
            break;
        }
        let t = (i..i + k).map(|j| s[j]).collect::<String>();
        *map.entry(t).or_insert(0) += 1;
    }

    let mx = map.values().copied().max().unwrap_or(0);
    let ans = map
        .iter()
        .filter(|&(_, v)| *v == mx)
        .map(|(k, _)| k.clone())
        .sorted()
        .collect_vec();
    println!("{}", mx);
    ans.iter().for_each(|s| {
        print!("{} ", s);
    });
}
