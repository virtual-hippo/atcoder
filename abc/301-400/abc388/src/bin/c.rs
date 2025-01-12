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
        a: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let bi = a.lower_bound(&(a[i] * 2));
        if bi < n {
            ans += n - bi;
        }
    }

    println!("{}", ans);
}
