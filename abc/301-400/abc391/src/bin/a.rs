#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn main() {
    input! {
        d: Chars,
    }
    let ans = d
        .iter()
        .map(|&ch| match ch {
            'N' => 'S',
            'E' => 'W',
            'W' => 'E',
            'S' => 'N',
            _ => unreachable!(),
        })
        .collect::<String>();

    println!("{}", ans);
}
