#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashSet, VecDeque};
use superslice::Ext;

fn f(
    graph: &Vec<Vec<(usize, u64)>>,
    n: usize,
    pos: usize,
    now: u64,
    ans: &mut u64,
    set: &mut HashSet<usize>,
) {
    if pos == n - 1 {
        if now < *ans {
            *ans = now;
        }
        return;
    }

    for &(next, label) in graph[pos].iter() {
        if set.contains(&next) {
            continue;
        }
        set.insert(next);
        f(graph, n, next, now ^ label, ans, set);
        set.remove(&next);
    }
}

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
            w: u64,
        }
        let a = a - 1;
        let b = b - 1;
        graph[a].push((b, w));
        graph[b].push((a, w));
    }

    let mut ans = u64::MAX;
    let mut set = HashSet::new();
    set.insert(0);
    f(&graph, n, 0, 0, &mut ans, &mut set);

    println!("{}", ans);
}
