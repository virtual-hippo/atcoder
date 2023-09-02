// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n + 1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: usize,
        }
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut ans = 0;
    for i in 1..n + 1 {
        let mut visited = vec![false; n + 1];
        ans = ans.max(dfs(&mut visited, &graph, i, 0));
    }
    println!("{}", ans);
}

fn dfs(
    visited: &mut Vec<bool>,
    graph: &Vec<Vec<(usize, usize)>>,
    pos: usize,
    current: usize,
) -> usize {
    visited[pos] = true;
    let mut ret = current;

    for (point, nagasa) in graph[pos].iter() {
        if visited[*point] == false {
            ret = ret.max(dfs(visited, graph, *point, current + nagasa));
            visited[*point] = false;
        }
    }
    ret
}

// fn bfs(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<i64> {
//     let mut dist = vec![-1_i64; graph.len()];
//     let mut queue = VecDeque::new();
//     queue.push_back(start);
//     dist[start] = 0;
//     while queue.is_empty() == false {
//         let pos = queue.pop_front().unwrap();
//         for (to, nagasa) in graph[pos].iter() {
//             if dist[*to] == -1 {
//                 dist[*to] = dist[pos] + *nagasa as i64;
//                 queue.push_back(*to);
//             }
//         }
//     }
//     dist
// }
