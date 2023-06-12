// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashMap;

fn dfs(visited: &mut HashMap<usize, bool>, graph: &HashMap<usize, Vec<usize>>, pos: usize) {
    visited.insert(pos, true);
    if !graph.contains_key(&pos) {
        return;
    }
    for point in graph[&pos].iter() {
        if visited[point] == false {
            dfs(visited, graph, *point);
        }
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut graph = HashMap::new();
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        graph.entry(a).or_insert_with(|| vec![]).push(b);
        graph.entry(b).or_insert_with(|| vec![]).push(a);
    }
    let mut visited = HashMap::new();

    for key in graph.keys() {
        visited.insert(*key, false);
    }

    dfs(&mut visited, &graph, 1);
    let mut ans = 1;
    for (key, val) in visited.iter() {
        if *val {
            ans = std::cmp::max(*key, ans);
        }
    }
    println!("{}", ans);
}

