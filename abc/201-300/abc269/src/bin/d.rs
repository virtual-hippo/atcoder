use proconio::input;

type Point = (i64, i64);

fn is_tonari(p1: Point, p2: Point) -> bool {
    p1 == (p2.0 - 1, p2.1 - 1) ||
    p1 == (p2.0 - 1, p2.1) ||
    p1 == (p2.0, p2.1 - 1) ||
    p1 == (p2.0 + 1, p2.1 + 1) ||
    p1 == (p2.0 + 1, p2.1) ||
    p1 == (p2.0, p2.1 + 1)
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        for j in i+1..n {
            if is_tonari(xy[i], xy[j]) {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }
    
    
    let mut count_connected = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            dfs(&mut visited, &graph, i);
            count_connected += 1;
        }
    }

    println!("{}", count_connected);
}

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize) {
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point);
        }
    }
}

