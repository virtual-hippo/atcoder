#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: i64,
        uv: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; 2 * n];
    for i in 0..m {
        let (u, v) = uv[i];
        let u = u - 1;
        let v = v - 1;

        graph[2 * u].push((2 * v, 1));
        graph[2 * v + 1].push((2 * u + 1, 1));
    }

    let mut heap = BinaryHeap::new();
    let mut kakutei = vec![false; n * 2];
    let mut costs = vec![i64::MAX; n * 2];
    costs[0] = 0;
    heap.push((Reverse(costs[0]), 0));
    while let Some((_, pos)) = heap.pop() {
        if kakutei[pos] {
            continue;
        }

        kakutei[pos] = true;
        for i in 0..graph[pos].len() {
            let next = graph[pos][i].0;
            let cost = graph[pos][i].1;
            if costs[next] > costs[pos] + cost {
                costs[next] = costs[pos] + cost;
                heap.push((Reverse(costs[next]), next));
            }
        }

        let next = pos ^ 1;
        let cost = x;
        if costs[next] > costs[pos] + cost {
            costs[next] = costs[pos] + cost;
            heap.push((Reverse(costs[next]), next));
        }
    }

    let ans = costs[2 * n - 1].min(costs[2 * n - 2]);
    println!("{}", ans);
}
