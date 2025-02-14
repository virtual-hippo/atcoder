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
        // N は区画の縦・横の数で、 N=50 を満たす。
        n: usize,
        // M はR国の人の数で、 50≤M≤1600 を満たす。
        m: usize,
        // K は鉄道会社Xの初期資金であり、 11000≤K≤20000 を満たす。
        k: usize,
        //T はゲームのターン数であり、 T=800 を満たす。
        t: usize,
    }
    let (home, workspace) = {
        let mut home = Vec::with_capacity(m);
        let mut workspace = Vec::with_capacity(m);

        for _ in 0..m {
            input! {
                i: usize,
                j: usize,
            }
            home.push((i, j));
            input! {
                i: usize,
                j: usize,
            }
            workspace.push((i, j));
        }

        (home, workspace)
    };

    let ans = 0;

    println!("{}", ans);
}
