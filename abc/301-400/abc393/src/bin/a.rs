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
        s1: String,
        s2: String,
    }
    if s1 == format!("sick") && s2 == format!("sick") {
        println!("{}", 1);
        return;
    }
    if s1 == format!("sick") {
        println!("{}", 2);
        return;
    }
    if s2 == format!("sick") {
        println!("{}", 3);
        return;
    }
    println!("{}", 4);
}
