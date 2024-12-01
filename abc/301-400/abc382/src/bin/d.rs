#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = vec![];
    for v in 1..=m - 10 {
        let mut now = vec![v];
        recursive(n, m, &mut now, v, &mut ans);
    }

    println!("{}", ans.len());
    for i in 0..ans.len() {
        for j in 0..ans[i].len() {
            print!("{} ", ans[i][j]);
        }
        println!("");
    }
}

fn recursive(n: usize, m: usize, now: &mut Vec<usize>, v: usize, ans: &mut Vec<Vec<usize>>) {
    if now.len() == n {
        ans.push(now.clone());
        return;
    }

    if v + 10 > m {
        return;
    }

    for vv in (v + 10)..=((m - 10 * (n - now.len() - 1)).max(v + 10)) {
        if vv > m {
            return;
        }
        now.push(vv);
        recursive(n, m, now, vv, ans);
        now.pop();
    }
}
