use proconio::{fastout, input};

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
            par: vec![n; n],
            siz: vec![1; n],
            sid: vec![0; n],
        }
    }

    fn root(&self, x: usize) -> usize {
        let mut current = x;
        loop {
            if self.par[current] == self.par.len() {
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
        m: usize,
        a: [usize; n],
    }
    let mut uf = UnionFind::new(n);
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        let u = u - 1;
        let v = v - 1;
        if a[u] == a[v] {
            uf.unite(u, v);
        } else if a[u] < a[v] {
            graph[u].push(v);
        } else {
            graph[v].push(u);
        }
    }
    let mut dp = vec![0; n];
    dp[uf.root(0)] = 1;
    let sorted_inds = {
        let mut ret = (0..n).collect::<Vec<usize>>();
        ret.sort_by(|&x, &y| a[x].cmp(&a[y]));
        ret
    };
    for &u in sorted_inds.iter() {
        for &v in graph[u].iter() {
            let u = uf.root(u);
            let v = uf.root(v);
            if dp[u] > 0 {
                dp[v] = dp[v].max(dp[u] + 1);
            }
        }
    }
    let ans = dp[uf.root(n - 1)].max(0);
    println!("{}", ans);
}
