use itertools::Itertools;
use proconio::input;

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

    fn same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
    }
    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
            w: u64,
        }
        edges.push((w % k, u, v));
    }

    let ans = (0..m)
        .combinations(n - 1)
        .filter_map(|comb| {
            let mut uf = UnionFind::new(n);
            let mut ret = 0;
            for i in comb {
                let (w, u, v) = edges[i];
                if uf.same(u, v) {
                    return None;
                }
                uf.unite(u, v);
                ret += w;
            }
            Some(ret % k)
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
