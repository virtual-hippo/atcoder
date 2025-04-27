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
        m: usize,
    }
    let mut shokuzai = vec![vec![]; n];
    let mut cnt = vec![0; m];

    for i in 0..m {
        input! {
            k: usize,
        }
        cnt[i] = k;

        for _ in 0..k {
            input! {
                a: usize,
            }
            shokuzai[a - 1].push(i);
        }
    }

    let mut now = 0;
    for _ in 0..n {
        input! {
            b: usize,
        }
        let b = b - 1;
        for i in shokuzai[b].iter() {
            cnt[*i] -= 1;
            if cnt[*i] == 0 {
                now += 1;
            }
        }
        println!("{}", now);
    }
}
