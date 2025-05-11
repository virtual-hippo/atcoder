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
        r: usize,
        x: usize,
    }

    if x == 1 {
        if 1600 <= r && r <= 2999 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if 1200 <= r && r <= 2399 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
