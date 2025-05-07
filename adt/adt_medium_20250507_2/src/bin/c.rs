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
        m: usize,
        t: usize,
        a: [usize; n-1]
    }
    let mut heya = vec![0; n];
    for _ in 0..m {
        input! {
            x: usize,
            y: usize,
        }
        let x = x - 1;
        heya[x] = y;
    }

    let mut now = t;
    for i in 0..n - 1 {
        if now > a[i] {
            now -= a[i];
            now += heya[i + 1];
        } else {
            println!("No");
            return;
        }
        eprintln!("{}", now);
    }

    println!("Yes");
}
