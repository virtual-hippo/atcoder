#![allow(unused_imports)]
use itertools::*;
use proconio::{fastout, input};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        _l: usize,
        t: [i64; n],
    }

    let sorted_t: Vec<(usize, i64)> = t
        .iter()
        .enumerate()
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .map(|(i, &v)| (i, v))
        .collect();

    let mut ans = vec![n; n * 2];
    ans[0] = sorted_t[0].0;
    ans[1] = sorted_t[0].0;
    for i in 0..n - 1 {
        let now = sorted_t[i].0;
        let to = sorted_t[i + 1].0;

        let (idx0, idx1) = if i % 2 == 0 {
            (2 * now, 2 * now + 1)
        } else {
            (2 * now + 1, 2 * now)
        };

        ans[idx0] = to;
        if (i + 1) % 25 == 0 {
            ans[idx1] = sorted_t[(i + 1) - 25].0;
        } else if (i + 1) % 50 == 0 {
            ans[idx1] = sorted_t[0].0;
        } else {
            ans[idx1] = to;
        }
    }

    let now = sorted_t[n - 1].0;
    ans[2 * now] = sorted_t[0].0;
    ans[2 * now + 1] = sorted_t[0].0;

    // -----------------------------------------------------------

    for i in 0..n {
        println!("{} {}", ans[2 * i], ans[2 * i + 1]);
    }
    let gosa = _get_score(&ans, n, &t);
    eprintln!("{}", 1_000_000 - gosa);
}

fn _get_score(answer: &Vec<usize>, n: usize, t: &Vec<i64>) -> i64 {
    let mut cnt = vec![0_i64; n];
    cnt[0] += 1;
    _dfs(answer, &mut cnt, 0, 0);
    (0..n).map(|i| (cnt[i] - t[i]).abs()).sum()
}

fn _dfs(graph: &Vec<usize>, cnt: &mut Vec<i64>, depth: usize, v: usize) {
    if depth == 500_000 {
        return;
    }
    cnt[v / 2] += 1;
    let u = if cnt[v] % 2 == 0 {
        graph[v * 2]
    } else {
        graph[v * 2 + 1]
    };
    _dfs(graph, cnt, depth + 1, u);
}
