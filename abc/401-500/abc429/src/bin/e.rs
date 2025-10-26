use proconio::{fastout, input, marker::Usize1};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut dist = vec![vec![]; graph.len()];
    let mut queue = VecDeque::new();

    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        graph[u].push(v);
        graph[v].push(u);
    }
    input! {
        s: String,
    }

    for (i, ch) in s.chars().enumerate() {
        if ch == 'S' {
            queue.push_back((i, i, 0));
        }
    }

    while let Some((v, sv, d)) = queue.pop_front() {
        for &u in graph[v].iter() {
            push(&mut queue, &mut dist, u, sv, d + 1);
        }
    }

    for (i, ch) in s.chars().enumerate() {
        if ch == 'D' {
            println!("{}", dist[i][0].0 + dist[i][1].0);
        }
    }
}

fn push(queue: &mut VecDeque<(usize, usize, usize)>, dist: &mut Vec<Vec<(usize, usize)>>, v: usize, sv: usize, d: usize) {
    if dist[v].len() >= 2 {
        return;
    }

    if dist[v].len() == 1 && dist[v][0].1 == sv {
        return;
    }

    dist[v].push((d, sv));
    queue.push_back((v, sv, d));
}
