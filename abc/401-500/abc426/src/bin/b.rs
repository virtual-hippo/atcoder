#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut map = HashMap::new();

    for ch in s.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }

    for (ch, &v) in map.iter() {
        if v == 1 {
            println!("{}", ch);
        }
    }
}
