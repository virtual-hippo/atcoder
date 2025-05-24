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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if a > c {
        println!("Yes");
        return;
    }
    if a < c {
        println!("No");
        return;
    }
    if b > d {
        println!("Yes");
        return;
    }
    println!("No");
}
