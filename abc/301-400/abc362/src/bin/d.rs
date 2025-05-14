#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let graph = {
        let mut graph = vec![vec![]; n];
        for _ in 0..m {
            input! {
                u: usize,
                v: usize,
                b: usize,
            }
            graph[u - 1].push((v - 1, b));
            graph[v - 1].push((u - 1, b));
        }
        graph
    };

    let mut heap = BinaryHeap::new();
    let mut kakutei = vec![false; n];
    let mut costs = vec![usize::MAX; n];

    costs[0] = a[0];

    heap.push((Reverse(costs[0]), 0));
    while let Some((_, pos)) = heap.pop() {
        if kakutei[pos] {
            continue;
        }

        kakutei[pos] = true;
        for i in 0..graph[pos].len() {
            let next = graph[pos][i].0;
            let cost = graph[pos][i].1 + a[next];
            if costs[next] > costs[pos] + cost {
                costs[next] = costs[pos] + cost;
                heap.push((Reverse(costs[next]), next));
            }
        }
    }

    for i in 1..n {
        if i == n - 1 {
            print!("{}\n", costs[i]);
        } else {
            print!("{} ", costs[i]);
        }
    }
}
