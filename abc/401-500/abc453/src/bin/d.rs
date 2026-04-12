use itertools::iproduct;
use proconio::{input, marker::*};
use std::collections::VecDeque;

const DIRS: [(char, i32, i32); 4] = [('D', 1, 0), ('U', -1, 0), ('R', 0, 1), ('L', 0, -1)];

fn neighbors(r: usize, c: usize, h: usize, w: usize) -> impl Iterator<Item = (usize, usize, usize)> {
    DIRS.iter().enumerate().filter_map(move |(di, &(_, dr, dc))| {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if 0 <= nr && nr < h as i32 && 0 <= nc && nc < w as i32 {
            Some((di, nr as usize, nc as usize))
        } else {
            None
        }
    })
}

fn push(r: usize, c: usize, dir: usize, s: &[Vec<char>], visited: &mut [Vec<[u8; 4]>], que: &mut VecDeque<(usize, usize, usize)>) {
    let h = s.len();
    let w = s[0].len();
    for (ndi, nr, nc) in neighbors(r, c, h, w) {
        if s[r][c] == 'o' && ndi != dir {
            continue;
        }
        if s[r][c] == 'x' && ndi == dir {
            continue;
        }
        if s[nr][nc] == '#' || visited[nr][nc][ndi] != u8::MAX {
            continue;
        }
        visited[nr][nc][ndi] = dir as u8;
        que.push_back((ndi, nr, nc));
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let (sr, sc) = iproduct!(0..h, 0..w).find(|&(i, j)| s[i][j] == 'S').unwrap();

    let mut visited = vec![vec![[u8::MAX; 4]; w]; h];
    let mut que = VecDeque::new();

    for di in 0..4 {
        push(sr, sc, di, &s, &mut visited, &mut que);
    }

    while let Some((dir, r, c)) = que.pop_front() {
        if s[r][c] == 'G' {
            println!("Yes");

            let mut path = Vec::new();
            let (mut cr, mut cc, mut cd) = (r, c, dir);
            while (cr, cc) != (sr, sc) {
                path.push(DIRS[cd].0);
                let prev_dir = visited[cr][cc][cd];
                cr = (cr as i32 - DIRS[cd].1) as usize;
                cc = (cc as i32 - DIRS[cd].2) as usize;
                cd = prev_dir as usize;
            }

            let ans: String = path.iter().rev().collect();
            println!("{}", ans);
            return;
        }

        push(r, c, dir, &s, &mut visited, &mut que);
    }

    println!("No");
}
