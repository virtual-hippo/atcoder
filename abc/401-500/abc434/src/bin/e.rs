use proconio::{fastout, input};
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::collections::*;

fn dfs(to: &HashMap<i64, Vec<i64>, FxBuildHasher>, used: &mut HashSet<i64, FxBuildHasher>, u: i64, n: &mut usize, m: &mut usize) {
    used.insert(u);
    *n += 1;

    for &v in to[&u].iter() {
        *m += 1;
        if used.contains(&v) {
            continue;
        }
        dfs(to, used, v, n, m);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut to = FxHashMap::default();
    for _ in 0..n {
        input! {
            x: i64,
            r: i64,
        }

        let u = x - r;
        let v = x + r;
        to.entry(u).or_insert_with(|| vec![]).push(v);
        to.entry(v).or_insert_with(|| vec![]).push(u);
    }

    let mut used = FxHashSet::default();
    let mut ans = 0;

    for &u in to.keys() {
        if used.contains(&u) {
            continue;
        }
        let mut n = 0;
        let mut m = 0;
        dfs(&to, &mut used, u, &mut n, &mut m);
        let m = m / 2;

        ans += if m < n { m } else { n };
    }

    println!("{}", ans);
}
