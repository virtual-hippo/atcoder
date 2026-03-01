// https://atcoder.jp/contests/abc447/tasks/abc447_f

use proconio::{input, marker::*};

fn dfs(to: &[Vec<usize>], v: usize, p: usize, ans: &mut usize) -> usize {
    let deg = to[v].len();

    let mut mx = 0;

    for &u in to[v].iter() {
        if u == p {
            continue;
        }
        let r = dfs(to, u, v, ans);
        if deg >= 4 {
            *ans = (*ans).max(mx + r + 1);
        }
        if deg >= 3 {
            *ans = (*ans).max(r + 1);
        }

        mx = mx.max(r);
    }

    match deg {
        4.. => mx + 1,
        3 => 1,
        _ => 0,
    }
}

fn solve() {
    input! {
        n: usize,
    }

    let mut to = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            a: Usize1,
            b: Usize1,
        }
        to[a].push(b);
        to[b].push(a);
    }

    let to = to;

    let mut ans = 1;
    dfs(&to, 0, usize::MAX, &mut ans);

    println!("{}", ans);
}

fn main() {
    input! {
        q: usize,
    }

    for _ in 0..q {
        solve();
    }
}
