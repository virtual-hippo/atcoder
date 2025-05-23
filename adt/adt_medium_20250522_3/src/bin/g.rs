use proconio::{fastout, input};

fn dfs(graph: &Vec<Vec<usize>>, start: usize, visited: &mut Vec<bool>, now: &mut Vec<usize>) {
    visited[start] = true;
    now.push(start);
    for &next in graph[start].iter() {
        if !visited[next] {
            dfs(graph, next, visited, now);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![vec![]; n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        let a = a - 1;
        let b = b - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = vec![false; n];
    for i in 0..n {
        let mut now = vec![];
        if !visited[i] {
            dfs(&graph, i, &mut visited, &mut now);
        }
        if now.len() < 2 {
            continue;
        }

        let mut counter = [0, 0, 0];
        for i in now.iter().map(|&x| graph[x].len()).map(|x| if x < 3 { x - 1 } else { 2 }) {
            counter[i] += 1;
        }

        let ok_val = [2, now.len() - 2, 0];
        if counter != ok_val {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
