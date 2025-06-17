use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut tree = vec![vec![]; n];

    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize,
        }
        let u = u - 1;
        let v = v - 1;
        tree[u].push(v);
        tree[v].push(u);
    }

    let mut visited = vec![false; n];
    let mut path = vec![];
    dfs(&tree, &mut visited, x - 1, &(y - 1), &mut false, &mut path);
    path.push(x - 1);

    let ans = path.iter().rev().map(|&u| format!("{}", u + 1)).join(" ");
    println!("{}", ans);
}

fn dfs(tree: &Vec<Vec<usize>>, visited: &mut Vec<bool>, u: usize, y: &usize, on_path: &mut bool, path: &mut Vec<usize>) {
    visited[u] = true;
    if u == *y {
        *on_path = true;
        return;
    }

    for &v in tree[u].iter() {
        if visited[v] {
            continue;
        }
        dfs(tree, visited, v, y, on_path, path);
        if *on_path {
            path.push(v);
            return;
        }
    }
}
