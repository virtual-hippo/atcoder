use proconio::{fastout, input};
use std::collections::*;
use std::ops::Bound::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        a: [i64; n],
    }

    let is_ok = |tree: &BTreeSet<i64>, x: i64| {
        let before = *tree.range((Unbounded, Included(x))).next_back().unwrap_or(&i64::MIN);
        let after = *tree.range((Included(x), Unbounded)).next().unwrap_or(&i64::MAX);
        before <= x - d && after >= x + d
    };

    let ans: usize = (0..n)
        .scan((0_usize, BTreeSet::new()), |(r, tree), l| {
            tree.insert(a[l]);
            *r = (*r).max(l + 1);

            while *r < n && is_ok(tree, a[*r]) {
                tree.insert(a[*r]);
                *r += 1;
            }

            let count = *r - l;
            tree.remove(&a[l]);
            Some(count)
        })
        .sum();

    println!("{}", ans);
}
