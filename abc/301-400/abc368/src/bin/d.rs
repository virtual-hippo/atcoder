use proconio::{fastout, input};

fn dfs(
    visited: &mut Vec<bool>,
    graph: &Vec<Vec<usize>>,
    pos: usize,
    count: &mut Vec<usize>,
    selected: &Vec<bool>,
) {
    visited[pos] = true;

    if selected[pos] {
        count[pos] += 1;
    }

    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point, count, selected);
            count[pos] += count[*point];
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut graph = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            a: usize,
            b: usize,
        }
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    input! {
        mut v: [usize; k],
    }
    let mut selected = vec![false; n];

    for i in 0..k {
        v[i] -= 1;
        selected[v[i]] = true;
    }

    let mut visited = vec![false; n];
    let mut count = vec![0; n];

    dfs(&mut visited, &graph, v[0], &mut count, &selected);

    let ans = count.iter().filter(|&&v| v > 0).count();
    println!("{}", ans);
}
