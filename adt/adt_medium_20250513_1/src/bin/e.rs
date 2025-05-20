use proconio::{fastout, input};

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize) {
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point);
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
            u: usize,
            v: usize,
        }
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let mut count_connected = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            dfs(&mut visited, &graph, i);
            count_connected += 1;
        }
    }

    if m == n - 1
        && count_connected == 1
        && visited.iter().all(|&x| x)
        && graph.iter().filter(|x| x.len() == 1).count() == 2
        && graph.iter().filter(|x| x.len() == 2).count() == n - 2
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
