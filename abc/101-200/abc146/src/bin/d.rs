use proconio::{fastout, input};
use std::collections::VecDeque;

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<i64> {
    let mut dist = vec![-1_i64; graph.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    dist[start] = 0;
    while queue.is_empty() == false {
        let pos = queue.pop_front().unwrap();
        for to in graph[pos].iter() {
            if dist[*to] == -1 {
                dist[*to] = dist[pos] + 1;
                queue.push_back(*to);
            }
        }
    }
    dist
}
struct UnionFind {
    /// 親頂点の番号
    par: Vec<usize>,
    /// 連結成分の頂点の個数
    siz: Vec<usize>,
    /// 連結成分の辺の個数
    sid: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: vec![0; n + 1],
            siz: vec![1; n + 1],
            sid: vec![0; n + 1],
        }
    }

    fn root(&self, x: usize) -> usize {
        let mut current = x;
        loop {
            if self.par[current] == 0 {
                break;
            }
            current = self.par[current];
        }
        current
    }

    fn unite(&mut self, u: usize, v: usize) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u == root_v {
            self.sid[root_u] += 1;
            return;
        }
        if self.siz[root_u] < self.siz[root_v] {
            self.par[root_u] = root_v;
            self.siz[root_v] += self.siz[root_u];
            self.sid[root_v] += 1;
            self.sid[root_v] += self.sid[root_u];
        } else {
            self.par[root_v] = root_u;
            self.siz[root_u] += self.siz[root_v];
            self.sid[root_u] += 1;
            self.sid[root_u] += self.sid[root_v];
        }
    }

    fn _same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut uf = UnionFind::new(n);
    let mut graph = vec![vec![]; n];
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        uf.unite(a, b);
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let root = uf.root(1);
    let ans = 0;
    println!("{}", ans);
}
