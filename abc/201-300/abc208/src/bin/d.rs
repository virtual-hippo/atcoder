use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut dist = vec![vec![u64::MAX; n]; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: u64,
        }
        dist[a - 1][b - 1] = c;
    }
    for i in 0..n {
        dist[i][i] = 0;
    }

    let mut ans = 0;
    for k in 0..n {
        for s in 0..n {
            for t in 0..n {
                dist[s][t] = dist[s][t].min(dist[s][k].saturating_add(dist[k][t]));
            }
        }
        ans += dist
            .iter()
            .flatten()
            .filter(|&&val| val != u64::MAX)
            .sum::<u64>();
    }
    println!("{}", ans);
}
