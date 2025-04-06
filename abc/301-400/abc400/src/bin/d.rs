#![allow(unused_imports)]
use ac_library::*;
use core::task;
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
        s: [Chars; h],
        takahashi: (usize,usize),
        sakanaya: (usize,usize),
    }
    let takahashi = (takahashi.0 - 1, takahashi.1 - 1);
    let sakanaya = (sakanaya.0 - 1, sakanaya.1 - 1);
    let d = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut dist = vec![vec![usize::MAX; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back(takahashi);
    dist[takahashi.0][takahashi.1] = 0;
    while let Some(pos) = queue.pop_front() {
        if pos == sakanaya {
            break;
        }

        let (x, y) = pos;
        for (dx, dy) in d.iter() {
            for j in 1..3 {
                let nx = x as isize + dx * j;
                let ny = y as isize + dy * j;
                if nx < 0 || ny < 0 || nx >= h as isize || ny >= w as isize {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;

                if s[nx][ny] == '#' {
                    if dist[nx][ny] > dist[x][y] + 1 {
                        dist[nx][ny] = dist[x][y] + 1;
                        queue.push_back((nx, ny));
                    }
                } else {
                    if j == 1 {
                        if dist[nx][ny] > dist[x][y] {
                            dist[nx][ny] = dist[x][y];
                            queue.push_front((nx, ny));
                        }
                    }
                }
            }
        }
    }

    println!("{}", dist[sakanaya.0][sakanaya.1]);
}
