#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let mut ans = vec![vec!['.'; n]; n];

    for x in 1..n + 1 {
        for y in 1..n + 1 {
            if x.min(y).min(n + 1 - x).min(n + 1 - y) % 4 == 1 {
                let (i, j) = (y, n + 1 - x);
                ans[i - 1][j - 1] = a[x - 1][y - 1];
            } else if x.min(y).min(n + 1 - x).min(n + 1 - y) % 4 == 2 {
                let (i, j) = (y, n + 1 - x);
                let (i, j) = (j, n + 1 - i);
                ans[i - 1][j - 1] = a[x - 1][y - 1];
            } else if x.min(y).min(n + 1 - x).min(n + 1 - y) % 4 == 3 {
                let (i, j) = (y, n + 1 - x);
                let (i, j) = (j, n + 1 - i);
                let (i, j) = (j, n + 1 - i);
                ans[i - 1][j - 1] = a[x - 1][y - 1];
            } else if x.min(y).min(n + 1 - x).min(n + 1 - y) % 4 == 0 {
                ans[x - 1][y - 1] = a[x - 1][y - 1];
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!("{}", ans[i][j])
        }
        println!("");
    }
}
