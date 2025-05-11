// 多始点 BFS

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
        s: [Chars; h],
    }

    let mut queue = VecDeque::new();
    let mut dist = vec![vec![(usize::MAX, '_'); w]; h];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'E' {
                dist[i][j] = (0, 'E');
                queue.push_back(Pos(i, j));
            }
            if s[i][j] == '#' {
                dist[i][j] = (0, '#');
            }
        }
    }

    while let Some(pos) = queue.pop_front() {
        for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            if !can_move_pos(&s, pos, &dir) {
                continue;
            }

            let Pos(i, j) = pos;

            let (ii, jj) = match dir {
                Dir::Up => (i - 1, j),
                Dir::Down => (i + 1, j),
                Dir::Left => (i, j - 1),
                Dir::Right => (i, j + 1),
            };

            if dist[ii][jj].0 == usize::MAX {
                dist[ii][jj] = (dist[i][j].0 + 1, dir.into_char());
                queue.push_back(Pos(ii, jj));
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", dist[i][j].1);
        }
        println!();
    }
}

pub fn can_move_pos(grid: &Vec<Vec<char>>, pos: Pos, dir: &Dir) -> bool {
    let Pos(i, j) = pos;
    let h = grid.len();
    let w = grid[0].len();

    match dir {
        Dir::Up => i > 0 && grid[i - 1][j] == '.',
        Dir::Down => i < h - 1 && grid[i + 1][j] == '.',
        Dir::Left => j > 0 && grid[i][j - 1] == '.',
        Dir::Right => j < w - 1 && grid[i][j + 1] == '.',
    }
}

#[derive(Clone, Copy)]
pub struct Pos(usize, usize);

#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    pub fn into_char(&self) -> char {
        match self {
            Dir::Up => 'v',
            Dir::Down => '^',
            Dir::Left => '>',
            Dir::Right => '<',
        }
    }
    pub fn from_char(c: char) -> Self {
        match c {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid direction"),
        }
    }
}
impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_char())
    }
}
