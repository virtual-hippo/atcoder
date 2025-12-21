use itertools::*;
use proconio::{fastout, input};
use std::collections::BTreeMap;

fn dfs(tree: &Vec<BTreeMap<usize, usize>>, vs: &Vec<Vec<usize>>, ans: &mut Vec<usize>, v: usize) {
    for &i in vs[v].iter() {
        ans.push(i);
    }

    for (_, &u) in tree[v].iter() {
        dfs(tree, vs, ans, u);
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut tree = vec![BTreeMap::new()];
    let mut vid = vec![0];

    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
        }

        let v = vid[x];
        let is_contain_y = tree[v].contains_key(&y);

        let u = if is_contain_y { tree[v][&y] } else { tree.len() };

        if !is_contain_y {
            tree.push(BTreeMap::new());
            tree[v].insert(y, u);
        }

        vid.push(u);
    }

    let vs = (1..=n).fold(vec![vec![]; tree.len()], |mut acc, i| {
        acc[vid[i]].push(i);
        acc
    });

    let mut ans = vec![];
    dfs(&tree, &vs, &mut ans, 0);

    print_vec_1line(&ans);
}

#[fastout]
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
