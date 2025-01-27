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
        x: usize,
    }
    let mut n = 1;

    loop {
        let v = (1..=n).fold(1, |v1, v2| v1 * v2);
        if v == x {
            break;
        }
        n += 1;
    }

    println!("{}", n);
}
