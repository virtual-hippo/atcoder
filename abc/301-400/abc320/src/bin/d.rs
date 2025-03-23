#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            x: i64,
            y: i64,
        }
        let a = a - 1;
        let b = b - 1;
        graph[a].push((b, (x, y)));
        graph[b].push((a, (-x, -y)));
    }

    let undecidable = (i64::MAX, i64::MAX);
    let mut positions = vec![undecidable; n];
    positions[0] = (0, 0);
    let mut visited = vec![false; n];
    dfs(&graph, 0, &mut visited, &mut positions);

    for &(x, y) in positions.iter() {
        if (x, y) == undecidable {
            println!("undecidable");
        } else {
            println!("{} {}", x, y);
        }
    }
}

fn dfs(
    graph: &Vec<Vec<(usize, (i64, i64))>>,
    v: usize,
    visited: &mut Vec<bool>,
    positions: &mut Vec<(i64, i64)>,
) {
    visited[v] = true;
    for &(u, (x, y)) in graph[v].iter() {
        if visited[u] {
            continue;
        }
        positions[u] = (positions[v].0 + x, positions[v].1 + y);
        dfs(graph, u, visited, positions);
    }
}
