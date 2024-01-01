use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    let ans = (1..n.len() / 2 + 1)
        .flat_map(|i| {
            (0..n.len()).combinations(i).map(|vec| {
                let mut left = vec![];
                let mut right = vec![];
                for j in 0..n.len() {
                    if vec.contains(&j) {
                        left.push(n[j]);
                    } else {
                        right.push(n[j]);
                    }
                }
                left.sort_by(|a, b| b.cmp(a));
                right.sort_by(|a, b| b.cmp(a));
                let left = if let Ok(v) = left.iter().collect::<String>().parse::<u64>() {
                    v
                } else {
                    0
                };
                let right = if let Ok(v) = right.iter().collect::<String>().parse::<u64>() {
                    v
                } else {
                    0
                };
                left * right
            })
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
