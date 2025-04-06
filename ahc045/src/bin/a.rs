#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::source::line::LineSource;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{self, BufReader, Write};
use superslice::Ext;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Square {
    lx: usize,
    rx: usize,
    ly: usize,
    ry: usize,
}
impl Square {
    fn new(lx: usize, rx: usize, ly: usize, ry: usize) -> Self {
        Self { lx, rx, ly, ry }
    }
}

// UnionFind implementation as a struct
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    // Create a new UnionFind structure with n elements
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        UnionFind { parent, rank }
    }

    // Find the root of element x with path compression
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    // Union elements x and y by rank
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
    }

    // Check if elements x and y are in the same set
    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[derive(Clone)]
struct Input {
    n: usize,            // 都市の個数 n=800
    m: usize,            // 都市のグループの個数 1 <= m <= 400
    q: usize,            // クエリ回数 q=400
    l: usize,            // クエリを行う都市の集合のサイズの上限 3 <= l <= 15
    w: usize, // 都市の座標が含まれる長方形の幅や高さとして有り得る最大値 500 <= w <= 2500
    g: Vec<usize>, // 各グループに割り当てる都市の個数を表す配列 G
    square: Vec<Square>, // 各グループに割り当てる都市の個数を表す配列 G
}

impl Input {
    fn from_stdin() -> Self {
        let mut stdin = LineSource::new(BufReader::new(io::stdin()));
        macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            n: usize, // 都市の個数 n=800
            m: usize, // 都市のグループの個数 1 <= m <= 400
            q: usize, // クエリ回数 q=400
            l: usize, // クエリを行う都市の集合のサイズの上限 3 <= l <= 15
            w: usize, // 都市の座標が含まれる長方形の幅や高さとして有り得る最大値 500 <= w <= 2500 (これが大きいと正確な位置が特定しにくい)
        }
        let mut g = vec![0; m];
        for i in 0..m {
            input! {
                v: usize,
            }
            g[i] = v;
        }
        let mut square = vec![];
        for _ in 0..n {
            input! {
                lx: usize,
                rx: usize,
                ly: usize,
                ry: usize,
            }
            square.push(Square::new(lx, rx, ly, ry));
        }

        Self {
            n,
            m,
            q,
            l,
            w,
            g,
            square,
        }
    }
}

// Calculate minimum possible squared distance between two squares
fn min_possible_distance_squared(a: &Square, b: &Square) -> usize {
    let x_overlap = a.rx >= b.lx && a.lx <= b.rx;
    let y_overlap = a.ry >= b.ly && a.ly <= b.ry;

    if x_overlap && y_overlap {
        0 // Squares overlap
    } else if x_overlap {
        // Overlap in x but not y
        let dy = if a.ry < b.ly {
            b.ly - a.ry
        } else {
            a.ly - b.ry
        };
        dy * dy
    } else if y_overlap {
        // Overlap in y but not x
        let dx = if a.rx < b.lx {
            b.lx - a.rx
        } else {
            a.lx - b.rx
        };
        dx * dx
    } else {
        // No overlap
        let dx = if a.rx < b.lx {
            b.lx - a.rx
        } else {
            a.lx - b.rx
        };

        let dy = if a.ry < b.ly {
            b.ly - a.ry
        } else {
            a.ly - b.ry
        };

        dx * dx + dy * dy
    }
}

fn create_distance(input: &Input) -> Vec<Vec<usize>> {
    let mut dist = vec![vec![0; input.n]; input.n];
    for i in 0..input.n - 1 {
        for j in i + 1..input.n {
            dist[i][j] = min_possible_distance_squared(&input.square[i], &input.square[j]);
            dist[j][i] = dist[i][j];
        }
    }
    dist
}

// Main solver function
fn solve(input: &Input) -> (Vec<Vec<usize>>, Vec<Vec<(usize, usize)>>) {
    let distance_cache = create_distance(input);

    // Available vertices
    let mut available: HashSet<usize> = (0..input.n).collect();

    // Results: groups of vertices and their MST edges
    let mut vertex_groups: Vec<Vec<usize>> = Vec::with_capacity(input.m);
    let mut edge_groups: Vec<Vec<(usize, usize)>> = Vec::with_capacity(input.m);

    // Process each tree
    for &size in input.g.iter() {
        if size == 0 {
            vertex_groups.push(Vec::new());
            edge_groups.push(Vec::new());
            continue;
        }

        // Find a starting vertex that's most isolated
        let mut best_vertex = *available.iter().next().unwrap();
        let mut max_min_distance = 0;

        for &v in &available {
            let mut min_dist = usize::MAX;
            for &u in &available {
                if u != v {
                    let dist = distance_cache[u][v];
                    min_dist = min_dist.min(dist);
                }
            }
            if min_dist > max_min_distance {
                max_min_distance = min_dist;
                best_vertex = v;
            }
        }

        // Build the vertex group
        let mut group = Vec::with_capacity(size);
        group.push(best_vertex);
        available.remove(&best_vertex);

        // Add remaining vertices using nearest-neighbor
        while group.len() < size && !available.is_empty() {
            let mut best_next = None;
            let mut min_dist = usize::MAX;

            for &v in &available {
                let mut closest_dist = usize::MAX;
                for &u in &group {
                    let dist = distance_cache[u][v];
                    closest_dist = closest_dist.min(dist);
                }

                if closest_dist < min_dist {
                    min_dist = closest_dist;
                    best_next = Some(v);
                }
            }

            if let Some(v) = best_next {
                group.push(v);
                available.remove(&v);
            }
        }

        // Build MST for this group
        let edges = if size <= 50 {
            build_mst_kruskal(&group, &distance_cache)
        } else {
            build_mst_prim(&group, &distance_cache)
        };

        vertex_groups.push(group);
        edge_groups.push(edges);
    }

    (vertex_groups, edge_groups)
}

// Build MST using Kruskal's algorithm (better for sparse graphs)
fn build_mst_kruskal(vertices: &[usize], distance_cache: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    if vertices.len() <= 1 {
        return Vec::new();
    }

    // Create all edges
    let mut edges = Vec::new();
    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let u = vertices[i];
            let v = vertices[j];
            let weight = distance_cache[u][v];
            edges.push((u, v, weight));
        }
    }

    // Sort by weight
    edges.sort_by_key(|&(_, _, weight)| weight);

    // Build MST
    let mut uf = UnionFind::new(distance_cache.len());
    let mut result = Vec::with_capacity(vertices.len() - 1);

    for (u, v, _) in edges {
        if !uf.connected(u, v) {
            uf.union(u, v);
            result.push((u, v));

            if result.len() == vertices.len() - 1 {
                break;
            }
        }
    }

    result
}

// Build MST using Prim's algorithm (better for dense graphs)
fn build_mst_prim(vertices: &[usize], distance_cache: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    if vertices.len() <= 1 {
        return Vec::new();
    }

    // Set up priority queue
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Edge {
        to: usize,
        from: usize,
        weight: usize,
    }

    impl Ord for Edge {
        fn cmp(&self, other: &Self) -> Ordering {
            self.weight.cmp(&other.weight)
        }
    }

    impl PartialOrd for Edge {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut result = Vec::with_capacity(vertices.len() - 1);
    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();

    // Start with first vertex
    visited.insert(vertices[0]);

    // Add edges from first vertex
    for i in 1..vertices.len() {
        let weight = distance_cache[vertices[0]][vertices[i]];
        pq.push(Reverse(Edge {
            from: vertices[0],
            to: vertices[i],
            weight,
        }));
    }

    // Process edges
    while let Some(Reverse(edge)) = pq.pop() {
        if visited.contains(&edge.to) {
            continue;
        }

        visited.insert(edge.to);
        result.push((edge.from, edge.to));

        for &v in vertices {
            if !visited.contains(&v) {
                let weight = distance_cache[edge.to][v];
                pq.push(Reverse(Edge {
                    from: edge.to,
                    to: v,
                    weight,
                }));
            }
        }

        if visited.len() == vertices.len() {
            break;
        }
    }

    result
}

fn query(c: &[usize]) -> Vec<(usize, usize)> {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    print!("? {}", c.len());
    for &city in c {
        print!(" {}", city);
    }
    println!();
    std::io::stdout().flush().unwrap();
    input! {
        ab: [(usize,usize); c.len()-1], // 都市の組
    }
    ab
}

fn answer(groups: &[Vec<usize>], edges: &[Vec<(usize, usize)>]) {
    println!("!");
    for (i, group) in groups.iter().enumerate() {
        println!(
            "{}",
            group
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        for edge in &edges[i] {
            println!("{} {}", edge.0, edge.1);
        }
    }
    std::io::stdout().flush().unwrap();
}

fn _sample(input: &Input) {
    let centers = input
        .square
        .iter()
        .map(|Square { lx, rx, ly, ry }| ((lx + rx) / 2, (ly + ry) / 2))
        .collect::<Vec<(usize, usize)>>();

    let mut cities: Vec<usize> = (0..input.n).collect();
    cities.sort_by_key(|&i| centers[i]);

    let mut groups = Vec::new();
    let mut start_idx = 0;
    for &group_size in &input.g {
        groups.push(cities[start_idx..start_idx + group_size].to_vec());
        start_idx += group_size;
    }

    let mut edges: Vec<Vec<(usize, usize)>> = vec![Vec::new(); input.m];
    for k in 0..input.m {
        for i in (0..input.g[k] - 1).step_by(2) {
            if i < input.g[k] - 2 {
                let ret = query(&groups[k][i..i + 3]);
                edges[k].extend(ret);
            } else {
                edges[k].push((groups[k][i] as usize, groups[k][i + 1] as usize));
            }
        }
    }

    answer(&groups, &edges);
}

fn main() {
    let input = Input::from_stdin();
    // _sample(&input);
    let ans = solve(&input);
    answer(&ans.0, &ans.1);
}
