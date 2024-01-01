use num_integer::gcd;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashSet;

#[derive(Clone)]
struct UnionFind {
    /// 親頂点の番号
    par: Vec<Vec<(usize, usize)>>,
    /// 連結成分の頂点の個数
    siz: Vec<Vec<usize>>,
}

impl UnionFind {
    fn new(h: usize, w: usize) -> Self {
        UnionFind {
            par: vec![vec![(h, w); w]; h],
            siz: vec![vec![1; w]; h],
        }
    }

    fn root(&self, (row, col): (usize, usize)) -> (usize, usize) {
        let mut current = (row, col);
        loop {
            if self.par[current.0][current.1] == (self.par.len(), self.par[0].len()) {
                break;
            }
            current = self.par[current.0][current.1];
        }
        current
    }

    fn unite(&mut self, u: (usize, usize), v: (usize, usize)) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u == root_v {
            return;
        }
        if self.siz[root_u.0][root_u.1] < self.siz[root_v.0][root_v.1] {
            self.par[root_u.0][root_u.1] = root_v;
            self.siz[root_v.0][root_v.1] += self.siz[root_u.0][root_u.1];
        } else {
            self.par[root_v.0][root_v.1] = root_u;
            self.siz[root_u.0][root_u.1] += self.siz[root_v.0][root_v.1];
        }
    }

    fn _same(&self, u: (usize, usize), v: (usize, usize)) -> bool {
        self.root(u) == self.root(v)
    }
}

const MOD: usize = 998244353;
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut uf = UnionFind::new(h, w);
    for i in 0..h {
        for j in 0..w {
            if i < h - 1 && s[i][j] == '#' && s[i + 1][j] == '#' {
                uf.unite((i, j), (i + 1, j));
            }
            if j < w - 1 && s[i][j] == '#' && s[i][j + 1] == '#' {
                uf.unite((i, j), (i, j + 1));
            }
        }
    }
    let count_connected = (0..h)
        .flat_map(|i| (0..w).map(move |j| (i, j)))
        .filter(|&(i, j)| s[i][j] == '#')
        .filter(|&(i, j)| uf.root((i, j)) == (i, j))
        .count();

    let mut p = 0;
    let mut q = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                q += 1;
                let mut prev = HashSet::new();

                if 0 < i && s[i - 1][j] == '#' {
                    prev.insert(uf.root((i - 1, j)));
                }
                if i < h - 1 && s[i + 1][j] == '#' {
                    prev.insert(uf.root((i + 1, j)));
                }
                if 0 < j && s[i][j - 1] == '#' {
                    prev.insert(uf.root((i, j - 1)));
                }
                if j < w - 1 && s[i][j + 1] == '#' {
                    prev.insert(uf.root((i, j + 1)));
                }

                p += (count_connected as i32 - (prev.len() as i32 - 1)) as usize;
                p %= MOD;
            }
        }
    }

    let g = gcd(p, q);
    let ans = find_r(p / g, q / g);
    println!("{}", ans);
}

fn find_r(p: usize, q: usize) -> usize {
    let denominator = modular_inv(q, MOD);
    (p * denominator) % MOD
}

// モジュラ逆数を求める
fn modular_inv(a: usize, m: usize) -> usize {
    power(a, m - 2, m)
}

// aのb乗をmで割った余りを返す関数
fn power(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ret = 1;
    for i in 0..30 {
        let wari = 1 << i;
        if (b / wari) % 2 == 1 {
            ret = (ret * p) % m;
        }
        p = (p * p) % m;
    }
    ret
}
