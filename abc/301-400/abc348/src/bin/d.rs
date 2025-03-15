#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, usize};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        n: usize,
    }
    let kusuri = {
        let mut ret = vec![vec![0; w]; h];

        for _ in 0..n {
            input! {
                r: usize,
                c: usize,
                e: usize,
            }
            let r = r - 1;
            let c = c - 1;
            ret[r][c] = e;
        }
        ret
    };

    let mut dist = vec![vec![usize::MAX; w]; h];
    let mut queue = VecDeque::new();
    let start = {
        let mut ret = (0, 0);
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == 'S' {
                    ret = (i, j);
                }
            }
        }
        ret
    };
    let goal = {
        let mut ret = (0, 0);
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == 'T' {
                    ret = (i, j);
                }
            }
        }
        ret
    };
    queue.push_back((start, 0));
    dist[start.0][start.1] = 0;

    while let Some((pos, energy)) = queue.pop_front() {
        if pos == goal {
            println!("Yes");
            return;
        }
        let energy = if energy < kusuri[pos.0][pos.1] {
            kusuri[pos.0][pos.1]
        } else {
            energy
        };
        if energy == 0 {
            continue;
        }

        if pos.0 > 0 && a[pos.0 - 1][pos.1] != '#' {
            let to = (pos.0 - 1, pos.1);
            if dist[to.0][to.1] == usize::MAX
                || (dist[to.0][to.1] < energy - 1 && kusuri[to.0][to.1] < energy - 1)
            {
                dist[to.0][to.1] = energy - 1;
                queue.push_back((to, energy - 1));
            }
        }
        if pos.0 < h - 1 && a[pos.0 + 1][pos.1] != '#' {
            let to = (pos.0 + 1, pos.1);
            if dist[to.0][to.1] == usize::MAX
                || (dist[to.0][to.1] < energy - 1 && kusuri[to.0][to.1] < energy - 1)
            {
                dist[to.0][to.1] = energy - 1;
                queue.push_back((to, energy - 1));
            }
        }
        if pos.1 > 0 && a[pos.0][pos.1 - 1] != '#' {
            let to = (pos.0, pos.1 - 1);
            if dist[to.0][to.1] == usize::MAX
                || (dist[to.0][to.1] < energy - 1 && kusuri[to.0][to.1] < energy - 1)
            {
                dist[to.0][to.1] = energy - 1;
                queue.push_back((to, energy - 1));
            }
        }
        if pos.1 < w - 1 && a[pos.0][pos.1 + 1] != '#' {
            let to = (pos.0, pos.1 + 1);
            if dist[to.0][to.1] == usize::MAX
                || (dist[to.0][to.1] < energy - 1 && kusuri[to.0][to.1] < energy - 1)
            {
                dist[to.0][to.1] = energy - 1;
                queue.push_back((to, energy - 1));
            }
        }
    }
    println!("No");
}
