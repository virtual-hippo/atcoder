#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let a = {
        let mut ret = Vec::with_capacity(n);
        for _ in 0..n {
            input! {
                k: usize,
                a: [usize; k],
            }
            let mut map = FxHashMap::default();
            for i in 0..k {
                *map.entry(a[i]).or_insert(0) += 1;
            }

            let mut map2 = FxHashMap::default();
            for (&key, &v) in map.iter() {
                map2.insert(key, (v as f64) / (k as f64));
            }
            ret.push(map2);
        }
        ret
    };

    let mut ans = 0.0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let mut val = 0.0;
            for (k, &v) in a[i].iter() {
                if let Some(&v2) = a[j].get(k) {
                    val += v * v2;
                }
            }

            if ans <= val {
                ans = val;
            }
        }
    }

    println!("{}", ans);
}
