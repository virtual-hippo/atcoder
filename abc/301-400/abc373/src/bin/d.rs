use proconio::{fastout, input};

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
            w: i64,
        }
        graph[u - 1].push((v - 1, w));
        graph[v - 1].push((u - 1, -w));
    }

    let mut visited = vec![false; n];
    let mut ans = vec![0; n];

    for i in 0..n {
        if !visited[i] {
            dfs(&mut visited, &mut ans, &graph, i);
        }
    }

    for i in 0..n {
        print!("{} ", ans[i]);
    }
}

fn dfs(visited: &mut Vec<bool>, ans: &mut Vec<i64>, graph: &Vec<Vec<(usize, i64)>>, pos: usize) {
    visited[pos] = true;
    for (point, w) in graph[pos].iter() {
        if visited[*point] == false {
            ans[*point] = *w + ans[pos];
            dfs(visited, ans, graph, *point);
        }
    }
}
