// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize) {
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n+1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        graph[a].push(b);
    }
    let mut ans = 0;
    for i in 1..n+1 {
        let mut visited = vec![false; n+1];
        dfs(&mut visited, &graph, i);
        ans += visited.iter().filter(|x| **x).count();
    }
    println!("{}", ans);
}

