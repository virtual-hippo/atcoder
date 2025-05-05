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
        n: usize,
        _m: usize,
        q: usize,
    }
    let mut is_ok = vec![FxHashSet::default(); n];
    let mut is_all_ok = vec![false; n];

    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                let x = x - 1;
                let y = y - 1;
                is_ok[x].insert(y);
            },
            2 => {
                input! {
                    x: usize,
                }
                let x = x - 1;
                is_all_ok[x] = true;
            },
            3 => {
                input! {
                    x: usize,
                    y: usize,
                }
                let x = x - 1;
                let y = y - 1;
                if is_all_ok[x] || is_ok[x].contains(&y) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            },
            _ => {},
        }
    }
}
