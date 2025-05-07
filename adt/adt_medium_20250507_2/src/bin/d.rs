#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet, VecDeque};
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
            x: usize,
            y: usize,
        }
        let x = x - 1;
        let y = y - 1;
        graph[x].push(y);
        graph[y].push(x);
    }

    for i in 0..n {
        graph[i].sort();
        print!("{} ", graph[i].len());
        for j in 0..graph[i].len() {
            print!("{} ", graph[i][j] + 1);
        }
        println!();
    }
}
