#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, usize};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
    }
    let t = t - 1;
    let tree = {
        let mut res = vec![vec![]; n];

        for _ in 0..n - 1 {
            input! {
                a: usize,
                b: usize,
            }
            let a = a - 1;
            let b = b - 1;
            res[a].push(b);
            res[b].push(a);
        }
        res
    };

    let mut ans = vec![0; n];
    dfs(t, usize::MAX, &tree, &mut ans);
    let ans = ans.iter().map(|v| v.to_string()).join(" ");
    println!("{}", ans);
}

fn dfs(v: usize, p: usize, tree: &Vec<Vec<usize>>, ans: &mut Vec<usize>) -> usize {
    let mut mx = 0;
    for &u in tree[v].iter().filter(|&&u| u != p) {
        mx = mx.max(dfs(u, v, tree, ans));
    }
    ans[v] = mx;
    mx + 1
}
