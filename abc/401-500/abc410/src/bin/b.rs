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
        n: usize,
        q: usize,
        x: [usize; q],
    }

    let mut bx = vec![0; n + 1];
    let mut ans = vec![];
    let mut mn = 1;

    for i in 0..q {
        if x[i] == 0 {
            bx[mn] += 1;
            ans.push(mn);
        } else {
            bx[x[i]] += 1;
            ans.push(x[i]);
        }

        for v in (1..n + 1).rev() {
            if bx[v] <= bx[mn] {
                mn = v;
            }
        }
    }

    for i in 0..q {
        print!("{} ", ans[i]);
    }
}
