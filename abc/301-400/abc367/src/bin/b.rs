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
        mut n: Chars,
    }

    let mut tail = n.len() - 1;
    while n[tail] == '0' {
        n.pop();
        tail -= 1;
    }
    if n[tail] == '.' {
        n.pop();
    }

    let ans = n.into_iter().collect::<String>();
    println!("{}", ans);
}
