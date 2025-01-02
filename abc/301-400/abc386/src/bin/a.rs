#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        a: [usize; 4],
    }
    let mut map = FxHashMap::default();
    for i in 0..4 {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    if map.len() == 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
