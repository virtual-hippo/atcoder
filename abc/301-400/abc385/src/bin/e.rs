#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut garph = vec![vec![]; n];

    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize,
        }
        let u = u - 1;
        let v = v - 1;
        garph[u].push(v);
        garph[v].push(u);
    }
    let deg: Vec<usize> = garph.iter().map(|v| v.len()).collect();

    let mut ans = n;
    // 中心を全探索
    for v in 0..n {
        let mut d: Vec<usize> = garph[v].iter().map(|j| deg[*j]).collect();
        d.sort_by(|a, b| b.cmp(a));

        // 大きい順に試す
        for i in 0..d.len() {
            let now = d[i] * (i + 1) + 1;
            ans = ans.min(n - now);
        }
    }

    println!("{}", ans);
}
