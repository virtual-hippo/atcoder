// 木の重心を使うのがポイント
#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let graph = {
        let mut graph = vec![vec![]; n];
        for _ in 0..n - 1 {
            input! {
                a: usize,
                b: usize,
            }
            graph[a - 1].push(b - 1);
            graph[b - 1].push(a - 1);
        }
        graph
    };

    input! {
        c: [u64; n],
    }
    let total = c.iter().sum::<u64>();

    let mut x = usize::MAX;
    dfs(&graph, usize::MAX, 0, &total, &mut x, &c);

    let mut ans = 0;
    dfs2(&graph, usize::MAX, x, 0, &mut ans, &c);
    println!("{}", ans);
}

// 木の重心を求める
fn dfs(
    graph: &Vec<Vec<usize>>,
    // 親頂点
    parent: usize,
    u: usize,
    // 木全体の頂点の重みの総和
    total: &u64,
    // 重心
    x: &mut usize,
    // 頂点の重み
    c: &Vec<u64>,
) -> u64 {
    let mut ret = c[u];
    let mut mx = 0;
    for &v in &graph[u] {
        if v == parent {
            continue;
        }
        // 部分木のサイズ
        let now = dfs(graph, u, v, total, x, c);
        ret += now;
        mx = mx.max(now);
    }
    // 親頂点の部分木のサイズ
    let parent_size = *total - ret;
    mx = mx.max(parent_size);
    if mx * 2 <= *total {
        *x = u;
    }
    ret
}

fn dfs2(graph: &Vec<Vec<usize>>, parent: usize, u: usize, dist: u64, ans: &mut u64, c: &Vec<u64>) {
    *ans += c[u] * dist;
    for &v in &graph[u] {
        if v == parent {
            continue;
        }
        dfs2(graph, u, v, dist + 1, ans, c);
    }
}
