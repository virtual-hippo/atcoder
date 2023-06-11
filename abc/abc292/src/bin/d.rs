// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
    sid: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: vec![0; n+1],
            siz: vec![1; n+1],
            sid: vec![0; n+1],
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

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        uf.unite(u,v);
    }
    let mut ans = true;
    for i in 1..=n {
        let root_i = uf.root(i);
        if uf.siz[root_i] != uf.sid[root_i] {
            ans = false;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

