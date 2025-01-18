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
        k: usize,
    }

    let mut p = vec![vec![0; n + 1]; 60];
    for i in 1..=n {
        input! {
            x: usize,
        }
        p[0][i] = x;
    }
    let a = {
        let mut ret = vec![0; n + 1];
        for i in 1..=n {
            input! {
                a: usize,
            }
            ret[i] = a;
        }
        ret
    };

    for lv in 1..60 {
        for i in 1..=n {
            p[lv][i] = p[lv - 1][p[lv - 1][i]];
        }
    }

    let mut q = (0..n + 1).collect_vec();
    let mut k = k;
    for lv in 0..60 {
        if k % 2 == 1 {
            for i in 1..=n {
                q[i] = p[lv][q[i]];
            }
        }
        k /= 2;
    }

    for i in 1..=n {
        print!("{} ", a[q[i]]);
    }
}
