use proconio::input;
use std::collections::VecDeque;

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<i64> {
    let mut dist = vec![-1_i64; graph.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    dist[start] = 0;
    while queue.is_empty() == false {
        let pos = queue.pop_front().unwrap();
        for to in graph[pos].iter() {
            if dist[*to] == -1 {
                dist[*to] = dist[pos] + 1;
                queue.push_back(*to);
            }
        }
    }
    dist
}

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n1 + n2 + 1];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        graph[a].push(b);
        graph[b].push(a);
    }
    let dist1 = bfs(&graph, 1);
    let dist2 = bfs(&graph, n1 + n2);
    println!(
        "{}",
        dist1.iter().max().unwrap() + dist2.iter().max().unwrap() + 1
    );
}
