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
        s: Chars,
    }
    let a = s[0] as u8 - b'0';
    let b = s[2] as u8 - b'0';

    let ans = a * b;
    println!("{}", ans);
}
