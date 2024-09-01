// ワーシャル-フロイド法

use itertools::Itertools;
use proconio::{fastout, input};

fn solve(dist: &Vec<Vec<i64>>, k: usize, b: &Vec<usize>, edge: &Vec<(usize, usize, i64)>) -> i64 {
    let mut ret = i64::MAX / 2;
    for bb in b.iter().permutations(k) {
        for bit in 0..(1 << k) {
            let mut here = 0;
            let mut now = 0;
            for j in 0..k {
                let (u, v, t) = edge[*bb[j]];
                let (to, from) = if bit & (1 << j) == 0 { (u, v) } else { (v, u) };
                now += dist[here][from] + t;
                here = to;
            }
            now += dist[here][dist.len() - 1];
            ret = ret.min(now);
        }
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut bridge = vec![];
    let mut dist = vec![vec![i64::MAX / 2; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
            t: i64,
        }
        bridge.push((u - 1, v - 1, t));

        dist[u - 1][v - 1] = dist[u - 1][v - 1].min(t);
        dist[v - 1][u - 1] = dist[v - 1][u - 1].min(t);
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j])
            }
        }
    }

    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            k: usize,
        }
        let mut b = vec![];
        for _ in 0..k {
            input! {
                bb: usize,
            }
            b.push(bb - 1);
        }
        println!("{}", solve(&dist, k, &b, &bridge));
    }
}
