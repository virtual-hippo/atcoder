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
        x: f64,
    }
    if x >= 38.0 {
        println!("1");
    } else if x >= 37.5 {
        println!("2");
    } else {
        println!("3");
    }
}
