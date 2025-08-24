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

    match s.as_str() {
        "red" => {
            println!("SSS");
        },
        "blue" => {
            println!("FFF");
        },
        "green" => {
            println!("MMM");
        },
        _ => {
            println!("Unknown");
        },
    }
}
