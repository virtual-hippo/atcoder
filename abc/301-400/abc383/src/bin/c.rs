#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        d: i64,
        mut s: [Chars; h],
    }

    let mut queue = VecDeque::new();
    let mut dist = vec![vec![-1; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
                queue.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }

    bfs(&s, &mut dist, &mut queue);

    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if dist[i][j] <= d && dist[i][j] != -1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn bfs(graph: &Vec<Vec<char>>, dist: &mut Vec<Vec<i64>>, queue: &mut VecDeque<(usize, usize)>) {
    while queue.is_empty() == false {
        let (i, j) = queue.pop_front().unwrap();
        for dir in [Dir::UP, Dir::DOWN, Dir::LEFT, Dir::RIGHT] {
            if !can_move(&graph, (i, j), &dir) {
                continue;
            }

            let (ii, jj) = match dir {
                Dir::UP => (i - 1, j),
                Dir::DOWN => (i + 1, j),
                Dir::LEFT => (i, j - 1),
                Dir::RIGHT => (i, j + 1),
            };

            if dist[ii][jj] == -1 {
                dist[ii][jj] = dist[i][j] + 1;
                queue.push_back((ii, jj));
            }
        }
    }
}

enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn can_move(s: &Vec<Vec<char>>, (i, j): (usize, usize), dir: &Dir) -> bool {
    match dir {
        Dir::UP => i > 0 && s[i - 1][j] != '#',
        Dir::DOWN => i < s.len() - 1 && s[i + 1][j] != '#',
        Dir::LEFT => j > 0 && s[i][j - 1] != '#',
        Dir::RIGHT => j < s[0].len() - 1 && s[i][j + 1] != '#',
    }
}
