#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        h: usize,
        _w: usize,
        x: usize,
        y: usize,
        s: [Chars; h],
        t: Chars,
    }
    let x = x - 1;
    let y = y - 1;
    let mut pos = (x, y);

    let mut set = HashSet::new();

    for ch in t.iter() {
        let can = match *ch {
            'U' => can_move(&s, pos, Dir::UP),
            'D' => can_move(&s, pos, Dir::DOWN),
            'L' => can_move(&s, pos, Dir::LEFT),
            'R' => can_move(&s, pos, Dir::RIGHT),
            _ => unreachable!(),
        };
        if can {
            match *ch {
                'U' => pos.0 -= 1,
                'D' => pos.0 += 1,
                'L' => pos.1 -= 1,
                'R' => pos.1 += 1,
                _ => unreachable!(),
            };
            if s[pos.0][pos.1] == '@' {
                set.insert(pos);
            }
        }
    }
    println!("{} {} {}", pos.0 + 1, pos.1 + 1, set.len());
}

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
