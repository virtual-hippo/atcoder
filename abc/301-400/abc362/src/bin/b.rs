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
        a: (i32, i32),
        b: (i32, i32),
        c: (i32, i32),
    }

    let ok = ((a.1 - b.1).abs() * (c.1 - b.1).abs() == (a.0 - b.0).abs() * (c.0 - b.0).abs())
        || ((a.1 - c.1).abs() * (b.1 - c.1).abs() == (a.0 - c.0).abs() * (b.0 - c.0).abs())
        || ((b.1 - a.1).abs() * (c.1 - a.1).abs() == (b.0 - a.0).abs() * (c.0 - a.0).abs());

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
