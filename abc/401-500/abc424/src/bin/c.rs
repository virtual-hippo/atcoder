use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ab = vec![];
    let mut graph = vec![vec![]; n + 1];
    for u in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        graph[a].push(u + 1);
        graph[b].push(u + 1);
        ab.push((a, b));
    }

    let mut visited = vec![false; n + 1];
    for i in 0..n {
        if !visited[i + 1] && (ab[i].0 == 0 && ab[i].1 == 0) {
            dfs(&mut visited, &graph, i + 1);
        }
    }

    let ans = visited.iter().filter(|&&x| x).count();
    println!("{}", ans);
}

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize) {
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point);
        }
    }
}
