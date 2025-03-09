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
    let mut a = vec![0; 100];

    for _ in 0..q {
        input! {
            type_: usize,
        }
        if type_ == 1 {
            input! {
                x: usize,
            }
            a.push(x);
        } else {
            let v = a.pop().unwrap();
            println!("{}", v);
        }
    }
}
