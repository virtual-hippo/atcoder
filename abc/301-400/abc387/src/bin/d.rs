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
        s: [Chars; h],
    }

    let start = (0..h)
        .flat_map(|i| (0..w).map(move |j| (i, j)))
        .find(|&(i, j)| s[i][j] == 'S')
        .unwrap();
    let goal = (0..h)
        .flat_map(|i| (0..w).map(move |j| (i, j)))
        .find(|&(i, j)| s[i][j] == 'G')
        .unwrap();

    let mut ans = i64::MAX;
    for v in [0, 1] {
        let mut dist = vec![vec![-1_i64; w]; h];
        let mut queue = VecDeque::new();
        queue.push_back(start);
        dist[start.0][start.1] = 0;
        while queue.is_empty() == false {
            let pos = queue.pop_front().unwrap();
            let dirs = if dist[pos.0][pos.1] % 2 == v {
                [Dir::UP, Dir::DOWN]
            } else {
                [Dir::LEFT, Dir::RIGHT]
            };
            let nexts: Vec<(usize, usize)> = dirs
                .iter()
                .filter(|dir| can_move(&s, pos, **dir))
                .map(|dir| match dir {
                    Dir::UP => (pos.0 - 1, pos.1),
                    Dir::DOWN => (pos.0 + 1, pos.1),
                    Dir::LEFT => (pos.0, pos.1 - 1),
                    Dir::RIGHT => (pos.0, pos.1 + 1),
                })
                .filter(|&np| dist[np.0][np.1] == -1)
                .collect();
            nexts.iter().for_each(|&np| {
                dist[np.0][np.1] = dist[pos.0][pos.1] + 1;
                queue.push_back(np);
            });
        }

        if dist[goal.0][goal.1] > 0 {
            ans = ans.min(dist[goal.0][goal.1]);
        }
    }
    if ans == i64::MAX {
        ans = -1;
    }
    println!("{}", ans);
}

#[derive(Copy, Clone)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn can_move(s: &Vec<Vec<char>>, (i, j): (usize, usize), dir: Dir) -> bool {
    match dir {
        Dir::UP => i > 0 && s[i - 1][j] != '#',
        Dir::DOWN => i < s.len() - 1 && s[i + 1][j] != '#',
        Dir::LEFT => j > 0 && s[i][j - 1] != '#',
        Dir::RIGHT => j < s[0].len() - 1 && s[i][j + 1] != '#',
    }
}
