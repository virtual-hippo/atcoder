#![allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
use std::{
    collections::{HashMap, HashSet},
    usize,
};

struct UnionFind {
    /// 親頂点の番号
    par: Vec<usize>,
    /// 連結成分の頂点の個数
    siz: Vec<usize>,
    /// 連結成分の辺の個数
    sid: Vec<usize>,
    /// 頂点ごとの子要素
    childs: Vec<Vec<usize>>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: vec![usize::MAX; n],
            siz: vec![1; n],
            sid: vec![0; n],
            childs: (0..n).map(|i| vec![i]).collect(),
        }
    }

    fn root(&self, x: usize) -> usize {
        let mut current = x;
        loop {
            if self.par[current] == usize::MAX {
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
        let (par, child) = if self.siz[root_u] < self.siz[root_v] {
            (root_v, root_u)
        } else {
            (root_u, root_v)
        };

        self.par[child] = par;
        self.siz[par] += self.siz[child];
        self.sid[par] += 1;
        self.sid[par] += self.sid[child];

        let len = self.childs[child].len();
        self.childs[child].sort_by(|a, b| b.cmp(a));
        (0..(len.min(10))).for_each(|i| {
            let v = self.childs[child][i];
            self.childs[par].push(v);
        });
        self.childs[par].sort_by(|a, b| b.cmp(a));
        while self.childs[par].len() > 10 {
            self.childs[par].pop();
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

    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        input! {
            qq: usize,
        }

        match qq {
            1 => {
                input! {
                    u: usize,
                    v: usize,
                }
                let u = u - 1;
                let v = v - 1;
                uf.unite(u, v);
            }
            2 => {
                input! {
                    v: usize,
                    k: usize,
                }
                let v = v - 1;
                let root = uf.root(v);
                if uf.childs[root].len() < k {
                    println!("{}", -1);
                } else {
                    println!("{}", uf.childs[root][k - 1] + 1);
                }
            }
            _ => unreachable!(),
        }
    }
}
