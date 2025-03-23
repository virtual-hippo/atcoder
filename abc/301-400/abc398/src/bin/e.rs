#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeSet, VecDeque};
use superslice::Ext;

use proconio::source::line::LineSource;
use std::io::{self, BufReader};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {
        n: usize,
    }

    let mut edges = FxHashSet::default();
    let mut graph = vec![vec![]; n];

    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize,
        }
        edges.insert((u, v));
        let u = u - 1;
        let v = v - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut colors = vec![0; n];
    dfs(&graph, 0, 0, 0, &mut colors);
    let c0 = (0..n).filter(|&i| colors[i] == 0).collect_vec();
    let c1 = (0..n).filter(|&i| colors[i] == 1).collect_vec();

    let mut candidates = FxHashSet::default();
    for i in c0.iter() {
        for j in c1.iter() {
            let edge: (usize, usize) = (i.min(j) + 1, j.max(i) + 1);
            if !edges.contains(&edge) {
                candidates.insert(edge);
            }
        }
    }

    let cnt = candidates.len();
    let s = if cnt % 2 == 1 { "First" } else { "Second" };
    println!("{}", s);

    let mut is_me = cnt % 2 == 1;

    loop {
        if is_me {
            let &(i, j) = candidates.iter().next().unwrap();
            println!("{} {}", i, j);
            candidates.remove(&(i, j));
        } else {
            input! {
                i: i64,
                j: i64,
            }
            if i == -1 && j == -1 {
                return;
            }
            candidates.remove(&(i as usize, j as usize));
        }
        is_me = !is_me;
    }
}

fn dfs(graph: &Vec<Vec<usize>>, u: usize, c: usize, parent: usize, colors: &mut Vec<usize>) {
    colors[u] = c;
    for &v in graph[u].iter().filter(|&&v| v != parent) {
        dfs(graph, v, c ^ 1, u, colors);
    }
}
