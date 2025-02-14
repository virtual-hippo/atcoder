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
    }
    let p = {
        let mut ret1 = vec![n; n];
        for i in 0..n {
            input! {
                q: usize,
            }
            ret1[i] = q - 1;
        }
        ret1
    };
    let q = {
        let mut ret1 = vec![n; n];
        for i in 0..n {
            input! {
                q: usize,
            }
            ret1[i] = q - 1;
        }
        ret1
    };

    let mut z = vec![n; n];
    for i in 0..n {
        z[q[i]] = i;
    }

    for i in 0..n {
        print!("{} ", q[p[z[i]]] + 1);
    }
}
