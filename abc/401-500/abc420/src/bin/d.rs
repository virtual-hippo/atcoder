#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let diff = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let start = iproduct!(0..h, 0..w).find(|&(i, j)| s[i][j] == 'S').unwrap();
    let goal = iproduct!(0..h, 0..w).find(|&(i, j)| s[i][j] == 'G').unwrap();

    let mut heap = BinaryHeap::new();
    let mut kakutei = vec![vec![[false; 2]; w]; h];
    let mut dist = vec![vec![[usize::MAX; 2]; w]; h];

    dist[start.0][start.1][0] = 0;

    heap.push((Reverse(dist[start.0][start.1][0]), start, 0));

    while let Some((_, pos, switched)) = heap.pop() {
        let b = switched;
        if kakutei[pos.0][pos.1][b] {
            continue;
        }
        kakutei[pos.0][pos.1][b] = true;

        for d in diff.iter() {
            let pos_i64 = (pos.0 as i64, pos.1 as i64);
            let new_pos = (pos_i64.0 + d.0, pos_i64.1 + d.1);
            if new_pos.0 > (h as i64 - 1) || new_pos.1 > (w as i64 - 1) || new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }

            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if s[new_pos.0][new_pos.1] == '#' {
                continue;
            }
            if switched == 0 && s[new_pos.0][new_pos.1] == 'x' {
                continue;
            }
            if switched == 1 && s[new_pos.0][new_pos.1] == 'o' {
                continue;
            }

            if s[new_pos.0][new_pos.1] == '?' {
                if dist[new_pos.0][new_pos.1][switched ^ 1] > dist[pos.0][pos.1][switched] + 1 {
                    dist[new_pos.0][new_pos.1][switched ^ 1] = dist[pos.0][pos.1][switched] + 1;
                    heap.push((Reverse(dist[new_pos.0][new_pos.1][switched ^ 1]), new_pos, switched ^ 1));
                }
            } else {
                if dist[new_pos.0][new_pos.1][switched] > dist[pos.0][pos.1][switched] + 1 {
                    dist[new_pos.0][new_pos.1][switched] = dist[pos.0][pos.1][switched] + 1;
                    heap.push((Reverse(dist[new_pos.0][new_pos.1][switched]), new_pos, switched));
                }
            }
        }
    }

    if dist[goal.0][goal.1][0] == usize::MAX && dist[goal.0][goal.1][1] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[goal.0][goal.1][0].min(dist[goal.0][goal.1][1]));
    }
}
