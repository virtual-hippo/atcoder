#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize) {
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point);
        }
    }
}

fn count_connected(n: usize, graph: &Vec<Vec<usize>>) -> usize {
    let mut count_connected = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            dfs(&mut visited, &graph, i);
            count_connected += 1;
        }
    }
    count_connected
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    if m != n {
        println!("No");
        return;
    }

    let mut counts = vec![0; n];
    let mut graph = vec![vec![]; n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        let a = a - 1;
        let b = b - 1;
        counts[a] += 1;
        counts[b] += 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let cnt = count_connected(n, &graph);
    let ok = counts.iter().all(|&x| x == 2) && cnt == 1;

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
