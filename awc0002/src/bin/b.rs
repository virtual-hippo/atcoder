// https://atcoder.jp/contests/awc0002/tasks/awc0002_b

use itertools::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [Usize1; m],
    }

    let b: HashSet<usize> = b.into_iter().unique().collect();

    let c = (0..n).filter(|&i| b.contains(&i) && a[i] < k).map(|i| a[i]).collect_vec();
    let ans: usize = c.iter().sum();
    println!("{} {}", c.len(), ans);
}
