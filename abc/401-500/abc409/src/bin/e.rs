use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut x: [i64; n],
    }

    let mut graph = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize,
            w: u64,
        }
        graph[u - 1].push((v - 1, w));
        graph[v - 1].push((u - 1, w));
    }

    let mut ans = 0;
    dfs(&graph, 0, usize::MAX, &mut ans, &x);

    println!("{}", ans);
}

fn dfs(graph: &Vec<Vec<(usize, u64)>>, u: usize, parent: usize, ans: &mut u64, x: &Vec<i64>) -> i64 {
    let mut total = x[u];
    for &(v, w) in graph[u].iter() {
        if v == parent {
            continue;
        }
        // 部分木の陽電子 + 電子の総和
        let r = dfs(graph, v, u, ans, x);
        *ans += w * (r.abs() as u64);
        total += r;
    }
    total
}
