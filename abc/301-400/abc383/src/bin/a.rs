#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        tv: [(usize, usize); n],
    }
    let mut ans = 0;
    for i in 0..n {
        if i > 0 {
            if tv[i].0 - tv[i - 1].0 >= ans {
                ans = 0;
            } else {
                ans -= (tv[i].0 - tv[i - 1].0);
            }
        }
        ans += tv[i].1;
    }
    println!("{}", ans);
}
