use proconio::input;

fn dfs(visited: &mut Vec<bool>, tabiji: &mut Vec<usize>, graph: &Vec<Vec<usize>>, pos: usize, prev: usize) {
    visited[pos] = true;
    tabiji.push(pos);
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, tabiji, graph, *point, pos);
        }
    }
    if prev != 0 {
        tabiji.push(prev);
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n+1];
    for _ in 0..n-1 {
        input! {
            a: usize,
            b: usize,
        }
        graph[a].push(b);
        graph[b].push(a);
    }
    for i in 0..n+1 {
        graph[i].sort();
    }
    let mut visited = vec![false; n+1];
    let mut tabiji = Vec::with_capacity(n);
    dfs(&mut visited, &mut tabiji, &graph, 1, 0);
    for toshi in tabiji.iter() {
        print!("{} ", toshi);
    }
}

