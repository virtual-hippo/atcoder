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
    let mut ans = vec![vec![' '; n]; n];

    for i in 0..n {
        let j = n - i;
        if i <= j {
            let ch = if i % 2 == 0 { '#' } else { '.' };
            for ii in i..j {
                for jj in i..j {
                    ans[ii][jj] = ch;
                }
            }
        }
    }
    for i in 0..n {
        println!("{}", ans[i].iter().collect::<String>());
    }
}
