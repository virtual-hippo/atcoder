use proconio::input;
use std::collections::HashSet;

fn dfs(set: &mut HashSet<usize>, graph: &Vec<Vec<usize>>, pos: usize) {
    set.remove(&pos);
    for point in graph[pos].iter() {
        if set.contains(&point) {
            dfs(set, graph, *point);
        }
    }
}
fn main() {
    input! {
        n: usize,
    }
    let mut a = Vec::with_capacity(n);
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            p: usize,
        }
        a.push(p);
        set.insert(p);
    }
    let k = set.len();
    let mut graph = vec![vec![]; 2 * 100_001];
    for i in 0..(n / 2 + 1) {
        if i >= (n / 2 + 1) {
            break;
        }
        graph[a[i]].push(a[n - 1 - i]);
        graph[a[n - 1 - i]].push(a[i]);
    }
    let mut count_connected = 0;
    for pos in a.iter() {
        if set.contains(pos) {
            dfs(&mut set, &graph, *pos);
            count_connected += 1;
        }
    }
    println!("{}", k - count_connected);
}
