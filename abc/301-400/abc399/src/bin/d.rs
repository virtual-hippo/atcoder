// 解説AC
#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        a: [usize; 2*n],
    }
    let position = (0..2 * n).fold(vec![vec![]; n + 1], |mut position, i| {
        position[a[i]].push(i);
        position
    });

    let pairs = (0..2 * n - 1)
        .map(|i| (a[i], a[i + 1]))
        .filter(|&(x, y)| {
            position[x][0] + 1 != position[x][1] && position[y][0] + 1 != position[y][1]
        })
        .filter(|&(x, y)| {
            position[x][0].abs_diff(position[y][0]) == 1
                && position[x][1].abs_diff(position[y][1]) == 1
        })
        .map(|(x, y)| (x.min(y), x.max(y)))
        .collect::<HashSet<(usize, usize)>>();

    let ans = pairs.len();
    println!("{}", ans);
}
