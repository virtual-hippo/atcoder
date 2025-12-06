use proconio::{fastout, input, marker::Chars};
use std::{collections::VecDeque, usize};

fn push(
    dist: &mut Vec<Vec<Vec<usize>>>,
    que: &mut VecDeque<(usize, usize, usize, usize)>,
    i: usize,
    j: usize,
    v: usize,
    d: usize,
    cost: usize,
) {
    let d = d + cost;
    if dist[i][j][v] <= d {
        return;
    }

    dist[i][j][v] = d;

    if cost == 0 {
        que.push_front((d, i, j, v));
    } else {
        que.push_back((d, i, j, v));
    }
}

fn solve(_: usize) {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let di: [i64; 4] = [-1, 0, 1, 0];
    let dj: [i64; 4] = [0, -1, 0, 1];

    let mut dist = vec![vec![vec![usize::MAX; 4]; w]; h];

    // VecDeque((距離, i,j,向き))
    let mut que = VecDeque::new();
    push(&mut dist, &mut que, 0, 0, 3, 0, 0);

    let mut ans = usize::MAX;

    while let Some((d, i, j, v)) = que.pop_front() {
        if dist[i][j][v] != d {
            continue;
        }

        let ov = if s[i][j] == 'B' {
            v ^ 1
        } else if s[i][j] == 'C' {
            v ^ 3
        } else {
            v
        };

        for u in 0..4 {
            let ni = i as i64 + di[u];
            let nj = j as i64 + dj[u];
            let cost = if ov == u { 0 } else { 1 };

            if ni == (h - 1) as i64 && nj == w as i64 {
                ans = ans.min(cost + d);
            }

            if ni < 0 || nj < 0 || ni >= h as i64 || nj >= w as i64 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;

            push(&mut dist, &mut que, ni, nj, u, d, cost);
        }
    }

    println!("{}", ans);
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    (0..t).for_each(solve);
}
