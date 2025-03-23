#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        a: [usize; 7],
    }
    let mut map: HashMap<usize, usize> = HashMap::default();
    for &v in a.iter() {
        *map.entry(v).or_insert(0) += 1;
    }

    let mut two = 0;
    let mut three = 0;
    for (_, v) in map.iter() {
        if *v > 2 {
            three += 1;
        } else if *v > 1 {
            two += 1;
        }
    }

    if three > 1 {
        println!("Yes");
    } else if three > 0 && two > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
