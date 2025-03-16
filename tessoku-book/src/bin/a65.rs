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
    }
    let tree = {
        let mut res = vec![vec![]; n];

        for i in 0..n - 1 {
            input! {
                a: usize,
            }
            let i = i + 1;
            let a = a - 1;
            res[a].push(i);
            res[i].push(a);
        }
        res
    };

    let mut buka = vec![1; n];
    dfs(0, usize::MAX, &tree, &mut buka);
    let ans = buka.iter().map(|&x| x.to_string()).join(" ");
    println!("{}", ans);
}

fn dfs(v: usize, p: usize, tree: &Vec<Vec<usize>>, buka: &mut Vec<usize>) -> usize {
    let mut res = 1;
    for &u in tree[v].iter().filter(|&&u| u != p) {
        buka[u] = v;
        res += dfs(u, v, tree, buka);
    }
    buka[v] = res - 1;
    res
}
