use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize,
        }
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    if graph[0].len() == 1 {
        println!("1");
        return;
    }

    let mut vec = vec![];
    for &point in graph[0].iter() {
        let mut len = 0;
        let mut visited = vec![false; n];
        visited[0] = true;
        rec(&graph, point, &mut len, &mut visited);
        vec.push(len);
    }
    vec.sort();
    let ans = (0..vec.len() - 1).fold(0, |sum, i| sum + vec[i]) + 1;
    println!("{}", ans);
}

fn rec(graph: &Vec<Vec<usize>>, target: usize, len: &mut usize, visited: &mut Vec<bool>) {
    *len += 1;
    visited[target] = true;
    for &point in graph[target].iter() {
        if visited[point] {
            continue;
        }
        rec(graph, point, len, visited);
    }
}
