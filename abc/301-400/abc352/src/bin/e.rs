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
        m: usize,
    }
    let mut edge_list = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            c: usize,
            a: [usize; k],
        }
        for i in 1..k {
            edge_list.push(((a[0], a[i]), c));
        }
    }
    edge_list.sort_by(|(_, a), (_, b)| a.cmp(b));

    let mut ans = 0;
    let mut uf = UnionFind::new(n);
    for i in 0..edge_list.len() {
        let ((u, v), cost) = edge_list[i];
        if uf._same(u, v) == false {
            uf.unite(u, v);
            ans += cost;
        }
    }
    if uf.siz[uf.root(n)] != n {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
