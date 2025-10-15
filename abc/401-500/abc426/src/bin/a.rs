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
        x: String,
        y: String,
    }

    let v = ["Ocelot", "Serval", "Lynx"];

    let x = v.iter().position(|&v| v == x.as_str()).unwrap();
    let y = v.iter().position(|&v| v == y.as_str()).unwrap();

    if x >= y {
        println!("Yes");
    } else {
        println!("No");
    }
}
