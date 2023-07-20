// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut graph = vec![vec![]; n+1];
    for _ in 0..n-1 {
        input! {
            a: usize,
            b: usize,
        }
        graph[a].push(b);
        graph[b].push(a);
    }
    let edge = graph.iter().position(|vec| vec.len() == 1).unwrap();
    let mut visited = vec![-1; n+1];
    dfs(&mut visited, &graph, edge, 0);
    
    for _ in 0..q {
        input! {
            c: usize,
            d: usize,
        }
        if visited[c] == visited[d] {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}

fn dfs(visited: &mut Vec<i32>, graph: &Vec<Vec<usize>>, pos: usize, cnt: i32) {
    visited[pos] = cnt % 2;
    for point in graph[pos].iter() {
        if visited[*point] == -1 {
            dfs(visited, graph, *point, cnt+1);
        }
    }
}

