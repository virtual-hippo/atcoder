#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        map.insert(i + 1, 1_u64);
    }

    for _ in 0..q {
        input! {
            x: usize,
            y: usize,
        }

        let mut koho = vec![];
        for &k in map.keys() {
            if k <= x {
                koho.push(k);
            } else {
                break;
            }
        }

        let mut s = 0;
        for k in koho.iter() {
            if let Some(v) = map.remove(k) {
                *map.entry(y).or_insert(0) += v;
                s += v;
            }
        }

        println!("{}", s);
    }
}
