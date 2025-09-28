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
        s: (i64,i64),
        t: (i64,i64),
    }

    let dy = (s.1 - t.1).abs();

    fn f(i: i64, j: i64) -> i64 {
        if (i + j) % 2 == 0 {
            i
        } else {
            i - 1
        }
    }

    let dx = (f(s.0, s.1) - f(t.0, t.1)).abs();

    let ans = dy.max((dx + dy) / 2);
    println!("{}", ans);
}
