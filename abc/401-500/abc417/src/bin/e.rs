use itertools::*;
use proconio::{input, marker::*};

fn dfs(to: &Vec<Vec<usize>>, visited: &mut Vec<bool>, v: usize, y: usize, his: &mut Vec<usize>) -> bool {
    visited[v] = true;
    his.push(v);
    if v == y {
        return true;
    }

    for &u in to[v].iter() {
        if visited[u] {
            continue;
        }

        if dfs(to, visited, u, y, his) {
            return true;
        }
    }
    his.pop();
    false
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
    }

    let mut to = vec![vec![]; n];

    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        to[u].push(v);
        to[v].push(u);
    }

    for i in 0..n {
        to[i].sort();
    }

    let mut his = vec![];
    dfs(&to, &mut vec![false; n], x, y, &mut his);
    let ans = his.iter().copied().map(|v| v + 1).collect_vec();
    print_vec_1line(&ans);
}

fn main() {
    input! {
        t: usize,
    }
    (0..t).for_each(|_| solve());
}

pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
