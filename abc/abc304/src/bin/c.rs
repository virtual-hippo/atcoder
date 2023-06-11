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

fn is_contain(a1: i64,a2: i64,b1: i64,b2: i64,d: i64) -> bool {
    let d2 = (a1 - b1) * (a1 - b1) + (a2 - b2) * (a2 - b2);
    d2 <= (d * d)
}


fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64,i64); n],
    }
    let mut uf = UnionFind::new(n);
    for i in 0..n-1 {
        for j in i+1..n {
            if is_contain(xy[i].0, xy[i].1, xy[j].0, xy[j].1, d) {
                uf.unite(i+1, j+1);
            }
        }
    }
    let root_1 = uf.root(1);
    for i in 0..n {
        if root_1 == uf.root(i+1) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

