use itertools::*;
use proconio::{fastout, input, marker::Chars};
use std::{collections::VecDeque, u64};

// grid bfs

const DI: [i64; 4] = [-1, 0, 1, 0];
const DJ: [i64; 4] = [0, -1, 0, 1];

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut teleport = vec![vec![]; 26];

    iproduct!(0..h, 0..w)
        .filter(|&(i, j)| s[i][j] != '.' && s[i][j] != '#')
        .for_each(|(i, j)| {
            let v = s[i][j] as u8 - b'a';
            teleport[v as usize].push((i, j));
        });

    let mut dist = vec![vec![u64::MAX; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back((0_usize, 0_usize));
    dist[0][0] = 0;

    let mut visited = vec![false; 26];

    while let Some((i, j)) = queue.pop_front() {
        if (i, j) == (h - 1, w - 1) {
            break;
        }

        if s[i][j] != '.' && s[i][j] != '#' {
            let v = s[i][j] as u8 - b'a';

            if !visited[v as usize] {
                visited[v as usize] = true;
                for &(ni, nj) in teleport[v as usize].iter() {
                    if dist[ni][nj] == u64::MAX {
                        dist[ni][nj] = dist[i][j] + 1;
                        queue.push_back((ni, nj));
                    }
                }
            }
        }

        (0..4)
            .filter(|&k| {
                let di = DI[k];
                let dj = DJ[k];
                let ni = i as i64 + di;
                let nj = j as i64 + dj;
                if ni < 0 || nj < 0 {
                    return false;
                }

                let ni = ni as usize;
                let nj = nj as usize;

                if ni >= h || nj >= w {
                    return false;
                }
                s[ni][nj] != '#'
            })
            .map(|k| {
                let di = DI[k];
                let dj = DJ[k];
                let ni = i as i64 + di;
                let nj = j as i64 + dj;

                let ni = ni as usize;
                let nj = nj as usize;
                (ni, nj)
            })
            .for_each(|(ni, nj)| {
                if dist[ni][nj] == u64::MAX {
                    dist[ni][nj] = dist[i][j] + 1;
                    queue.push_back((ni, nj));
                }
            });
    }

    if dist[h - 1][w - 1] == u64::MAX {
        println!("{}", -1);
    } else {
        println!("{}", dist[h - 1][w - 1]);
    }
}
