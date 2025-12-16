#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut graph = vec![vec![]; n];

    for i in 0..n {
        input! {
            c: usize,
            p: [Usize1; c],
        }

        graph[i] = p;
    }

    let mut ans = vec![];
    dfs(&mut vec![false; n], &graph, 0, &mut ans);

    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, u: usize, ans: &mut Vec<usize>) {
    visited[u] = true;
    for v in graph[u].iter() {
        if visited[*v] == false {
            dfs(visited, graph, *v, ans);
        }
    }

    if u != 0 {
        ans.push(u + 1);
    }
}
