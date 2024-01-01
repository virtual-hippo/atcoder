use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map: BTreeMap<u64, i64> = BTreeMap::new();
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        *map.entry(a).or_insert(0) += 1;
        *map.entry(a + b).or_insert(0) -= 1;
    }
    let mut s = BTreeMap::new();
    {
        let mut prev = 0;
        for (&k, &v) in map.iter() {
            s.insert(k, v + prev);
            prev = v + prev;
        }
    }

    let mut ans = vec![0; n + 1];
    {
        for ((&k0, &v0), (&k1, &_v1)) in s.iter().tuple_windows() {
            // println!("({} {}) | ({} {})", k0, v0, k1, v1);
            ans[v0 as usize] += k1 - k0;
        }
    }
    for i in 1..=n {
        print!("{} ", ans[i]);
    }
}
