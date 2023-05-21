// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

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
        (n,m): (usize, usize),
        mut s: [Chars; n],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        for j in i+1..n {
            let mut cnt = 0;
            for k in 0..m {
                if s[i][k] != s[j][k] {
                    cnt +=1;
                }
            }
            if cnt == 1 {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }
    let mut count_connected = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            dfs(&mut visited, &graph, i);
            count_connected += 1;
        }
    }
    if count_connected == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
    
}

