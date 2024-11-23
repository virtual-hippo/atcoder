use proconio::{fastout, input};

struct UnionFind {
    /// 親頂点の番号
    par: Vec<usize>,
    /// 連結成分の頂点の個数
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: vec![0; n],
            siz: vec![1; n],
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
            return;
        }
        if self.siz[root_u] < self.siz[root_v] {
            self.par[root_u] = root_v;
            self.siz[root_v] += self.siz[root_u];
        } else {
            self.par[root_v] = root_u;
            self.siz[root_u] += self.siz[root_v];
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
        q: usize,
    }
    let mut uf = UnionFind::new(n + 2);
    let mut cnt = vec![1; n + 2];

    let mut l: Vec<usize> = (0..(n + 2)).map(|i| i).collect();
    let mut r: Vec<usize> = (0..(n + 2)).map(|i| i).collect();
    let mut color: Vec<usize> = (0..(n + 2)).map(|i| i).collect();

    for _ in 0..q {
        input! {
            query_type: usize,
        }
        match query_type {
            1 => {
                input! {
                    x: usize,
                    c: usize,
                }
                let x = uf.root(x);
                let siz = uf.siz[x];

                cnt[color[x]] -= siz;
                color[x] = c;
                cnt[color[x]] += siz;

                {
                    let li = uf.root(l[x] - 1);
                    if color[li] == c {
                        let pl = l[li];
                        let pr = r[x];

                        uf.unite(x, li);
                        let x = uf.root(x);
                        l[x] = pl;
                        r[x] = pr;
                        color[x] = c;
                    }
                }

                {
                    let x = uf.root(x);
                    let ri = uf.root(r[x] + 1);
                    if color[ri] == c {
                        let pl = l[x];
                        let pr = r[ri];

                        uf.unite(x, ri);
                        let x = uf.root(x);
                        l[x] = pl;
                        r[x] = pr;
                        color[x] = c;
                    }
                }
            }
            2 => {
                input! {
                    c: usize
                }
                println!("{}", cnt[c]);
            }
            _ => unreachable!(),
        }
    }
}
