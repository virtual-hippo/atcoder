#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut xs = vec![];
    let mut s = vec![];

    dfs(&a, 0, &mut xs, &mut s);

    let ans = xs.iter().unique().count();
    println!("{}", ans);
}

fn dfs(a: &Vec<usize>, i: usize, xs: &mut Vec<usize>, s: &mut Vec<usize>) {
    if i == a.len() {
        let val = s.iter().fold(0, |v, ret| v ^ ret);
        xs.push(val);

        return;
    }

    for j in 0..s.len() {
        s[j] += a[i];
        dfs(a, i + 1, xs, s);
        s[j] -= a[i];
    }

    s.push(a[i]);
    dfs(a, i + 1, xs, s);
    s.pop();
}
