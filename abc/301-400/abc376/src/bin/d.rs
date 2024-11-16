use proconio::{fastout, input};
use std::{
    collections::{HashSet, VecDeque},
    i64,
};

fn bfs(graph: &Vec<HashSet<usize>>, start: usize) -> Vec<i64> {
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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![HashSet::new(); n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        graph[a - 1].insert(b - 1);
    }
    let dists = bfs(&graph, 0);

    let ans = dists
        .iter()
        .enumerate()
        .filter(|(_, d)| **d > 0)
        .filter(|(i, _)| graph[*i].contains(&0))
        .fold(
            (0, i64::MAX - 1),
            |(pi, min), (i, &d)| if d < min { (i, d) } else { (pi, min) },
        )
        .1
        + 1;
    if ans == i64::MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
