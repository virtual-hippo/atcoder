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
        s: Chars,
        t: Chars,
    }

    let mut set = FxHashSet::default();

    for (i, ch) in s.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if ch.is_uppercase() {
            set.insert(s[i - 1]);
        }
    }

    for ch in set.iter() {
        if !t.contains(ch) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
