// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: vec![0; n+1],
            siz: vec![1; n+1],
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

    fn same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}

fn main() {
    input! {
        (n,q): (usize, usize),
    }
    let mut graph = UnionFind::new(n);
    for _i in 0..q {
        input! {
            (x,u,v): (usize, usize, usize),
        }
        if x == 1 {
            graph.unite(u,v);
        } else if x == 2 {
            if graph.same(u,v) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

