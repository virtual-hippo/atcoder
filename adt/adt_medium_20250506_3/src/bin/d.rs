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
        n: usize,
    }
    let mut a = vec![];

    for _ in 0..n {
        input! {
            c: usize,
            aa: [usize; c],
        }
        a.push(aa);
    }

    input! {
        x: usize,
    }
    let b = a
        .iter()
        .enumerate()
        .filter(|(_, v)| v.contains(&x))
        .collect::<Vec<(usize, &Vec<usize>)>>();
    if b.is_empty() {
        println!("0");
        return;
    }
    let min_v = b.iter().map(|(_, v)| v.len()).min().unwrap();
    let b = b
        .iter()
        .filter(|(_, v)| v.len() == min_v)
        .map(|(i, _)| i + 1)
        .collect::<Vec<usize>>();

    println!("{}", b.len());
    for val in b.iter() {
        print!("{} ", val);
    }
}
