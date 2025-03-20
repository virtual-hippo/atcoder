#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeMap;
use superslice::Ext;

#[fastout]
fn main() {
    solve1();
}

fn _solve0() {
    input! {
        n: usize,
    }
    let mut slime = BTreeMap::new();
    for _ in 0..n {
        input! {
            s: usize,
            c: u64,
        }
        slime.insert(s, c);
    }

    let mut nokori = BTreeMap::new();

    while let Some((siz, count)) = slime.pop_first() {
        if count > 1 {
            *slime.entry(siz * 2).or_insert(0) += count / 2;
        }
        if count % 2 == 1 {
            nokori.insert(siz, 1);
        }
    }
    let ans = nokori.len();

    println!("{}", ans);
}

fn solve1() {
    input! {
        n: usize,
    }
    let mut slime = BTreeMap::new();
    for _ in 0..n {
        input! {
            s: usize,
            c: u64,
        }
        let mut s = s;
        let mut x = 1;
        while s % 2 == 0 {
            s /= 2;
            x *= 2;
        }
        *slime.entry(s).or_insert(0) += c * x;
    }

    let ans: u64 = slime.iter().map(|(_, v)| v.count_ones() as u64).sum();

    println!("{}", ans);
}
