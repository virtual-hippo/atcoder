// ワーシャルフロイド

use itertools::*;
use proconio::{fastout, input, marker::Usize1};

fn update_dist(i: usize, j: usize, w: u64, n: usize, dist: &mut Vec<Vec<u64>>) {
    for (u, v) in iproduct!(0..n + 1, 0..n + 1) {
        if dist[u][i] == u64::MAX || dist[j][v] == u64::MAX {
            continue;
        }
        dist[u][v] = dist[u][v].min(dist[u][i] + w + dist[j][v]);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1,Usize1,u64); m],
        d: usize,
        t: u64,
        k: [Usize1; d],
        q: usize,
    }

    let mut dist = vec![vec![u64::MAX; n + 1]; n + 1];
    for i in 0..n + 1 {
        dist[i][i] = 0;
    }

    for i in 0..m {
        let (a, b, c) = abc[i];
        dist[a][b] = dist[a][b].min(c);
        dist[b][a] = dist[b][a].min(c);
    }

    for &u in k.iter() {
        dist[u][n] = dist[u][n].min(t);
        dist[n][u] = 0;
    }

    for i in 0..n + 1 {
        update_dist(i, i, 0, n, &mut dist);
    }

    for _ in 0..q {
        input! {
            q: usize,
        }

        match q {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                    c: u64,
                }
                if dist[a][b] <= c && dist[b][a] <= c {
                    continue;
                }

                dist[a][b] = dist[a][b].min(c);
                dist[b][a] = dist[b][a].min(c);

                update_dist(a, b, c, n, &mut dist);
                update_dist(b, a, c, n, &mut dist);
            },
            2 => {
                input! {
                    x: Usize1,
                }
                if t < dist[x][n] {
                    dist[x][n] = t.min(dist[x][n]);
                    update_dist(x, n, t, n, &mut dist);
                }

                dist[n][x] = 0;
                update_dist(n, x, 0, n, &mut dist);
            },
            3 => {
                let ans = iproduct!(0..n, 0..n)
                    .map(|(u, v)| dist[u][v])
                    .filter(|&x| x != u64::MAX)
                    .sum::<u64>();
                println!("{}", ans);
            },
            _ => unreachable!(),
        }
    }
}
