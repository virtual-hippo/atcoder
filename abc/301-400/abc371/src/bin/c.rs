use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mg: usize,
    }
    let mut g = vec![vec![false; n]; n];
    for _ in 0..mg {
        input! {
            u: usize,
            v: usize,
        }
        g[u - 1][v - 1] = true;
        g[v - 1][u - 1] = true;
    }
    input! {
        mh: usize,
    }

    let mut h = vec![vec![false; n]; n];
    for _ in 0..mh {
        input! {
            u: usize,
            v: usize,
        }
        h[u - 1][v - 1] = true;
        h[v - 1][u - 1] = true;
    }

    let mut a = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        for j in (i + 1)..n {
            input! {
                v: usize,
            }
            a[i][j] = v;
            a[j][i] = v;
        }
    }

    let mut ans = usize::MAX;

    for p in (0..n).permutations(n) {
        let mut now = 0;
        for i in 0..n - 1 {
            for j in (i + 1)..n {
                if g[i][j] != h[p[i]][p[j]] {
                    now += a[p[i]][p[j]];
                }
            }
        }
        ans = ans.min(now);
    }

    println!("{}", ans);
}
