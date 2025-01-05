#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use superslice::Ext;

#[fastout]
fn main() {
    let mut map = FxHashMap::default();
    for i in 0..5 {
        input! {
            v: usize,
        }
        map.insert(('A' as u8 + i) as char, v);
    }
    let c = vec!['A', 'B', 'C', 'D', 'E'];
    let mut ans = vec![];
    for n in 1..6 {
        for chars in c.iter().combinations(n) {
            let score: usize = chars.iter().map(|cc| *(map.get(cc).unwrap_or(&0))).sum();
            ans.push((
                Reverse(score),
                chars.iter().map(|ch| **ch).collect::<String>(),
            ));
        }
    }
    ans.sort();
    for i in 0..ans.len() {
        println!("{}", ans[i].1);
    }
}
