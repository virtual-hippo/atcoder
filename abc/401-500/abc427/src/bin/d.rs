use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    (0..t).for_each(|_| solve());
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Chars,
    }

    let mut g = vec![vec![]; n];

    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        g[u].push(v);
    }

    // dp[i][v] i 手目に 頂点 v だった際の勝者
    let mut dp = vec![vec![false; n]; 2 * k + 1];
    for v in 0..n {
        dp[2 * k][v] = s[v] == 'A';
    }

    for i in (0..(2 * k)).rev() {
        for u in 0..n {
            dp[i][u] = !(g[u].iter().all(|&v| dp[i + 1][v]));
        }
    }

    if dp[0][0] {
        println!("Alice");
    } else {
        println!("Bob");
    }
}
