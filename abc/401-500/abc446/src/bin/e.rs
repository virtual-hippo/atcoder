use itertools::*;
use proconio::input;
use std::collections::*;

fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    }

    let mut to = vec![vec![vec![]; m]; m];
    for i in 0..m {
        for j in 0..m {
            let k = (i * b + j * a) % m;
            to[j][k].push((i, j));
        }
    }

    let mut que = VecDeque::new();
    let mut seen = vec![vec![false; m]; m];
    for i in 0..m {
        let j = 0;
        seen[i][j] = true;
        que.push_back((i, j));
    }
    for j in 0..m {
        let i = 0;
        seen[i][j] = true;
        que.push_back((i, j));
    }

    while let Some((i, j)) = que.pop_front() {
        for &(ni, nj) in to[i][j].iter() {
            if !seen[ni][nj] {
                seen[ni][nj] = true;
                que.push_back((ni, nj));
            }
        }
    }
    let ans = iproduct!(0..m, 0..m).filter(|&(i, j)| !seen[i][j]).count();
    println!("{}", ans);
}
