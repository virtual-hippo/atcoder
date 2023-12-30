use proconio::{fastout, input};

fn calc_d(pos1: (i64, i64), pos2: (i64, i64)) -> i64 {
    (pos2.0 - pos1.0).pow(2) + (pos2.1 - pos1.1).pow(2)
}

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
        d: i64,
        xy: [(i64,i64); n],
    }
    let mut graph = vec![vec![]; n];
    let d2 = d * d;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if calc_d(xy[i], xy[j]) <= d2 {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }
    let mut visited = vec![false; n];
    dfs(&mut visited, &graph, 0);
    for val in visited {
        if val {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
