use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![vec![false; n]; n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        let a = a - 1;
        let b = b - 1;
        let (a, b) = if a > b { (b, a) } else { (a, b) };

        graph[a][b] = true;
    }

    let ans = (0..n)
        .permutations(n)
        .map(|ord| {
            let single_cycle_cost = {
                let g = (0..n)
                    .map(|i| {
                        let (u, v) = (ord[i], ord[(i + 1) % n]);
                        if u > v {
                            (v, u)
                        } else {
                            (u, v)
                        }
                    })
                    .fold(vec![vec![false; n]; n], |mut g, (u, v)| {
                        g[u][v] = true;
                        g
                    });
                iproduct!(0..n, 0..n).filter(|&(u, v)| graph[u][v] != g[u][v]).count()
            };

            let double_cycle_costs = (3..=n - 3).map(|d| {
                let g = (0..d)
                    .map(|i| {
                        let (u, v) = (ord[i], ord[(i + 1) % d]);
                        if u > v {
                            (v, u)
                        } else {
                            (u, v)
                        }
                    })
                    .chain((d..n).map(|i| {
                        let (u, v) = (ord[i], ord[(i + 1) % (n - d) + d]);
                        if u > v {
                            (v, u)
                        } else {
                            (u, v)
                        }
                    }))
                    .fold(vec![vec![false; n]; n], |mut g, (u, v)| {
                        g[u][v] = true;
                        g
                    });
                iproduct!(0..n, 0..n).filter(|&(u, v)| graph[u][v] != g[u][v]).count()
            });

            [single_cycle_cost].into_iter().chain(double_cycle_costs).min().unwrap()
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
