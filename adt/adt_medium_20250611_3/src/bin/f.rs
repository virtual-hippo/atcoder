use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut b_map = BTreeMap::new();
    for &v in a.iter() {
        *b_map.entry(v).or_insert(0) += 1;
    }

    let keys = b_map.keys().cloned().collect::<Vec<_>>();
    let values = b_map.values().cloned().collect::<Vec<_>>();

    let mut l = 0;
    let mut r = 0;
    let mut now = 0;
    let mut ans = 0;

    while l < keys.len() {
        while r < keys.len() && keys[r] < keys[l] + m {
            now += values[r];
            r += 1;
        }
        ans = ans.max(now);
        now -= values[l];
        l += 1;
    }

    println!("{}", ans);
}
