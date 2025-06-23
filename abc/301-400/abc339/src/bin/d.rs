use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: [Chars; n]
    }

    let mut p = vec![];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                p.push((i, j));
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                s[i][j] = '.';
            }
        }
    }

    let mut visited = vec![vec![vec![vec![false; n]; n]; n]; n];
    visited[p[0].0][p[0].1][p[1].0][p[1].1] = true;

    let dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

    let mut queue = VecDeque::new();
    queue.push_back((p[0], p[1], 0));

    while let Some((p0, p1, now_dist)) = queue.pop_front() {
        for &dir in dirs.iter() {
            let mut current_s = &mut s;
            current_s[p0.0][p0.1] = 'P';
            current_s[p1.0][p1.1] = 'P';

            let can_move_p0 = can_move(&current_s, p0, dir);
            let can_move_p1 = can_move(&current_s, p1, dir);

            let new_p0 = if can_move_p0 { move_pos(&mut current_s, p0, dir) } else { p0 };
            let new_p1 = if can_move_p1 { move_pos(&mut current_s, p1, dir) } else { p1 };

            if new_p0 == new_p1 {
                println!("{}", now_dist + 1);
                return;
            }

            if !visited[new_p0.0][new_p0.1][new_p1.0][new_p1.1] {
                visited[new_p0.0][new_p0.1][new_p1.0][new_p1.1] = true;
                queue.push_back((new_p0, new_p1, now_dist + 1));
            }

            current_s[new_p0.0][new_p0.1] = '.';
            current_s[new_p1.0][new_p1.1] = '.';
        }
    }

    let ans = -1;
    println!("{}", ans);
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn can_move(s: &Vec<Vec<char>>, (i, j): (usize, usize), dir: Dir) -> bool {
    match dir {
        Dir::Up => i > 0 && s[i - 1][j] != '#',
        Dir::Down => i < s.len() - 1 && s[i + 1][j] != '#',
        Dir::Left => j > 0 && s[i][j - 1] != '#',
        Dir::Right => j < s[0].len() - 1 && s[i][j + 1] != '#',
    }
}

fn move_pos(s: &mut Vec<Vec<char>>, (i, j): (usize, usize), dir: Dir) -> (usize, usize) {
    match dir {
        Dir::Up => {
            s[i][j] = '.';
            s[i - 1][j] = 'P';
            (i - 1, j)
        },
        Dir::Down => {
            s[i][j] = '.';
            s[i + 1][j] = 'P';
            (i + 1, j)
        },
        Dir::Left => {
            s[i][j] = '.';
            s[i][j - 1] = 'P';
            (i, j - 1)
        },
        Dir::Right => {
            s[i][j] = '.';
            s[i][j + 1] = 'P';
            (i, j + 1)
        },
    }
}
