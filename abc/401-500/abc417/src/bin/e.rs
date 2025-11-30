use proconio::marker::Usize1;
use proconio::{fastout, input};

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, u: usize, path: &mut Vec<usize>, y: usize) {
    if visited[y] {
        return;
    }
    visited[u] = true;

    if u == y {
        for i in 0..path.len() {
            if i < path.len() - 1 {
                print!("{} ", path[i] + 1);
            } else {
                println!("{}", path[i] + 1);
            }
        }
        return;
    }

    for &v in graph[u].iter() {
        if visited[v] {
            continue;
        }

        path.push(v);
        dfs(visited, graph, v, path, y);
        path.pop();
    }
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
    }

    let g = {
        let mut g = vec![vec![]; n];

        for _ in 0..m {
            input! {
                u: Usize1,
                v: Usize1,
            }
            g[u].push(v);
            g[v].push(u);
        }
        for u in 0..n {
            g[u].sort();
        }
        g
    };

    dfs(&mut vec![false; n], &g, x, &mut vec![x], y);
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    (0..t).for_each(|_| solve());
}
