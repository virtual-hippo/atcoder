#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let blacks = (0..h)
        .flat_map(|i| (0..w).map(move |j| (i, j)))
        .filter(|&(i, j)| s[i][j] == '#')
        .collect_vec();
    let top = blacks
        .iter()
        .map(|&(i, _)| i)
        .fold(h + 1, |v, ret| ret.min(v));
    let bottom = blacks.iter().map(|&(i, _)| i).fold(0, |v, ret| ret.max(v));

    let left = blacks
        .iter()
        .map(|&(_, j)| j)
        .fold(w + 1, |v, ret| ret.min(v));
    let right = blacks.iter().map(|&(_, j)| j).fold(0, |v, ret| ret.max(v));

    for i in top..bottom + 1 {
        for j in left..right + 1 {
            if s[i][j] == '.' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");

}
