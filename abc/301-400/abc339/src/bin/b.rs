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
        h: usize,
        w: usize,
        n: usize,
    }

    let mut grid = vec![vec![true; w]; h];

    let mut di = 0;
    let d = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut now = (0, 0);

    for _ in 0..n {
        if grid[now.0][now.1] {
            grid[now.0][now.1] = false;
            di = (4 + di + 1) % 4;
            now.0 += (h as i64 + d[di].0) as usize;
            now.1 += (w as i64 + d[di].1) as usize;
            now.0 %= h;
            now.1 %= w;
        } else {
            grid[now.0][now.1] = true;
            di = (4 + di - 1) % 4;
            now.0 += (h as i64 + d[di].0) as usize;
            now.1 += (w as i64 + d[di].1) as usize;
            now.0 %= h;
            now.1 %= w;
        }
    }

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}
