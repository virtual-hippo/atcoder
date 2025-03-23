#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeSet;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        r: i64,
        c: i64,
        s: Chars,
    }

    let mut set = FxHashSet::default();
    set.insert((0, 0));

    let mut sotai_takibi = (0_i64, 0_i64);
    let mut sotai_takahashi = (r, c);

    for i in 0..n {
        match s[i] {
            'N' => {
                sotai_takibi.0 += 1;
                sotai_takahashi.0 += 1;
            }
            'W' => {
                sotai_takibi.1 += 1;
                sotai_takahashi.1 += 1;
            }
            'S' => {
                sotai_takibi.0 -= 1;
                sotai_takahashi.0 -= 1;
            }
            'E' => {
                sotai_takibi.1 -= 1;
                sotai_takahashi.1 -= 1;
            }
            _ => unreachable!(),
        }
        set.insert(sotai_takibi);
        if set.contains(&sotai_takahashi) {
            print!("1");
        } else {
            print!("0");
        }
    }
}
