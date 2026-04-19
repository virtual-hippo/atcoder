use proconio::{input, marker::*};

fn dfs(g: &[Vec<usize>], seen: &mut [bool], u: usize) {
    seen[u] = true;
    for &v in g[u].iter() {
        if seen[v] {
            continue;
        }
        dfs(g, seen, v);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        g[a].push(b);
    }

    let mut seen = vec![false; n];
    dfs(&g, &mut seen, 0);

    let ans = seen.iter().filter(|&&v| v).count();
    println!("{}", ans);
}
