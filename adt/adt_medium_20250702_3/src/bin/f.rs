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
        n: i64,
        a: i64,
        b: i64,
        p: i64,
        q: i64,
        r: i64,
        s: i64,
    }

    let is_black = |i: i64, j: i64| -> bool {
        {
            let mn_k = (1 - a).max(1 - b);
            let mx_k = (n - a).min(n - b);
            if j - i == b - a && a + mn_k <= i && i <= a + mx_k && b + mn_k <= j && j <= b + mx_k {
                return true;
            }
        }

        {
            let mn_k = (1 - a).max(b - n);
            let mx_k = (n - a).min(b - 1);

            if i + j == a + b && a + mn_k <= i && i <= a + mx_k && b - mx_k <= j && j <= b - mn_k {
                return true;
            }
        }

        false
    };

    for i in p..=q {
        for j in r..=s {
            if is_black(i, j) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
