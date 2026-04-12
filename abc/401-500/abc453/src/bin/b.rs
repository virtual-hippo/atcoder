#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{input, marker::*};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::*;
use superslice::Ext;

fn main() {
    input! {
        t: usize,
        x: usize,
        a: [usize; t+1]
    }
    let mut s = a[0];
    println!("{} {}", 0, s);
    for i in 1..t + 1 {
        if a[i].abs_diff(s) >= x {
            s = a[i];
            println!("{} {}", i, s);
        }
    }
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
