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
        q: usize,
    }
    let mut que = VecDeque::new();
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: usize,
                }
                que.push_back(x);
            },
            2 => {
                if let Some(x) = que.pop_front() {
                    println!("{}", x);
                }
            },
            _ => {
                println!("Invalid query type");
            },
        }
    }
}
