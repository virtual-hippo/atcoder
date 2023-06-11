// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize, h: usize, is_keibi: &mut Vec<bool>) {
    visited[pos] = true;
    is_keibi[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false && h > 0{
            dfs(visited, graph, *point, h-1, is_keibi);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut graph = vec![vec![]; n+1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        graph[a].push(b);
        graph[b].push(a);
    }
    
    let mut is_keibi = vec![false; n+1];
    for _ in 0..k {
        input! {
            p: usize,
            mut h: usize,
        }
        let mut visited = vec![false; n+1];
        if !visited[k] {
            dfs(&mut visited, &graph, p, h, &mut is_keibi);
        }
    }
    let g = is_keibi.iter().filter(|x| **x).count();
    println!("{}", g);
    for i in 0..is_keibi.len() {
        if is_keibi[i] {
            print!("{} ", i)
        }
    }
}
