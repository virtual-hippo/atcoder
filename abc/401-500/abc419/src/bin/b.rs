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
        q: usize,
    }

    let mut count = vec![0; 102];

    for _ in 0..q {
        input! {
            query: usize,
        }
        if query == 1 {
            input! {
                x: usize,
            }
            count[x] += 1;
        } else {
            let ans = count.iter().position(|&c| c > 0).unwrap();
            println!("{}", ans);
            count[ans] -= 1;
        }
    }
}
