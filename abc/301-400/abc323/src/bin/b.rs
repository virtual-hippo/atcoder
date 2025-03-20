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
        s: [Chars; n],
    }

    let mut shori = vec![0; n];

    for i in 0..n - 1 {
        for j in i + 1..n {
            if s[i][j] == 'o' {
                shori[i] += 1;
            } else {
                shori[j] += 1;
            }
        }
    }
    let ans = shori
        .iter()
        .enumerate()
        .sorted_by(|(_, x), (_, y)| y.cmp(x))
        .map(|(i, _)| (i + 1).to_string())
        .join(" ");

    println!("{}", ans);
}
