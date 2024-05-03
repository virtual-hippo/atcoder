use proconio::{fastout, input};

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize, cnt: &mut usize) {
    *cnt += 1;
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point, cnt);
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
    let mut max_sid = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            let mut cnt = 0;
            dfs(&mut visited, &graph, i, &mut cnt);
            max_sid += (cnt * (cnt - 1)) / 2;
        }
    }

    let ans = max_sid - m;
    println!("{}", ans);
}
