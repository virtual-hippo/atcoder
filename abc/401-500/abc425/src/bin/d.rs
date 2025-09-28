use itertools::*;
use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut que = VecDeque::new();

    let mut ss = vec![vec![usize::MAX; w]; h];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                que.push_back(((i, j), 0));
                ss[i][j] = 0;
            }
        }
    }

    let d = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some(((i, j), t)) = que.pop_front() {
        for (di, dj) in d {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;

            if ss[ni][nj] <= t {
                continue;
            }

            let ok = d
                .iter()
                .filter(|&&(di, dj)| {
                    let ni = ni as isize + di;
                    let nj = nj as isize + dj;
                    if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                        return false;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    ss[ni][nj] <= t
                })
                .count()
                == 1;

            if ok {
                ss[ni][nj] = t + 1;
                que.push_back(((ni, nj), t + 1));
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == usize::MAX {
                eprint!(".");
            } else {
                eprint!("#");
            }
        }
        eprintln!();
    }

    let ans = iproduct!(0..h, 0..w)
        .map(|(i, j)| if ss[i][j] != usize::MAX { 1 } else { 0 })
        .sum::<u64>();
    println!("{}", ans);
}
