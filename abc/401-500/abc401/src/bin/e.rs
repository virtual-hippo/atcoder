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
    }

    let graph = {
        let mut graph = vec![vec![]; n];
        for _ in 0..m {
            input! {
                a: usize,
                b: usize,
            }
            let a = a - 1;
            let b = b - 1;
            graph[a].push(b);
            graph[b].push(a);
        }
        for i in 0..n {
            graph[i].sort();
        }
        graph
    };

    let mut set = FxHashSet::default();
    let mut dsu = Dsu::new(n);
    let mut ans = vec![-1; n];
    for i in 0..n {
        set.remove(&i);
        for &to in graph[i].iter() {
            if to < i {
                dsu.merge(i, to);
            } else {
                set.insert(to);
            }
        }
        if dsu.size(i) == i + 1 {
            ans[i] = set.len() as i64;
        }
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}
