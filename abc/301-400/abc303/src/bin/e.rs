use itertools::*;
use proconio::{fastout, input, marker::*};
use rustc_hash::{FxBuildHasher, FxHashSet};
use std::collections::*;

fn dfs(
    to: &Vec<HashSet<usize, FxBuildHasher>>,
    deg_cnt: &Vec<usize>,
    visited: &mut Vec<bool>,
    ans: &mut HashMap<usize, usize>,
    u: usize,
    parent: usize,
    depth: usize,
) {
    if depth % 3 == 2 {
        ans.insert(parent, deg_cnt[parent]);
    }

    visited[u] = true;
    for &v in to[u].iter() {
        if visited[v] {
            continue;
        }
        dfs(to, deg_cnt, visited, ans, v, u, depth + 1);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut to = vec![FxHashSet::default(); n];

    for _ in 0..n - 1 {
        input! {
            u: Usize1,
            v: Usize1,
        }
        to[u].insert(v);
        to[v].insert(u);
    }

    let deg_cnt = (0..n).map(|i| to[i].len()).collect_vec();
    let mut ans = HashMap::new();
    let start = (0..n).find(|&i| deg_cnt[i] == 1).unwrap();
    dfs(&to, &deg_cnt, &mut vec![false; n], &mut ans, start, usize::MAX, 0);
    let ans = ans.iter().map(|(_, &v)| v).sorted().collect_vec();

    print_vec_1line(&ans);
}

pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
