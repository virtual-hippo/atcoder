use itertools::Itertools;
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
    }
    let mut uf = UnionFind::new(n);
    let mut amari = vec![];

    for i in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        let a = a - 1;
        let b = b - 1;

        if uf._same(a, b) {
            amari.push((i, a, b));
        } else {
            uf.unite(a, b);
        }
    }
    let amari = {
        let mut ret = vec![vec![]; n];
        for (i, a, b) in amari.iter() {
            ret[uf.root(*a)].push((*i, *a, *b))
        }
        ret
    };

    let cc = (0..n)
        .filter(|&i| i == uf.root(i))
        .map(|i| (amari[i].len(), i))
        .sorted_by(|a, b| b.cmp(a))
        .collect_vec();

    let mut ans = vec![];
    let mut pos = 1;

    for i in 0..cc.len() {
        let (_, root) = cc[i];
        for (i, a, _) in amari[root].iter() {
            if pos < cc.len() {
                ans.push((i, a, cc[pos].1));
                pos += 1;
            }
        }
    }

    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{} {} {}", ans[i].0 + 1, ans[i].1 + 1, ans[i].2 + 1);
    }
}
