#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet, VecDeque};
use superslice::Ext;

fn char_to_index(ch: char) -> usize {
    ch as usize - 'a' as usize
}

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut t = t;
    if t[2] == 'X' {
        t.pop();
    }

    let mut ind = vec![vec![]; 26];

    for (i, ch) in s.iter().enumerate() {
        let ch = char_to_index(*ch);
        ind[ch].push(i);
    }

    let t = t
        .iter()
        .map(|ch| ch.to_ascii_lowercase())
        .map(|ch| char_to_index(ch))
        .collect::<Vec<usize>>();

    let is_ok = t.iter().map(|&i| ind[i].len()).all(|x| x > 0);
    if !is_ok {
        println!("No");
        return;
    }

    let v0 = ind[t[0]][0];
    let i1 = ind[t[1]].upper_bound(&v0);

    if i1 == ind[t[1]].len() {
        println!("No");
        return;
    }

    let v1 = ind[t[1]][i1];

    if v0 >= v1 {
        println!("No");
        return;
    }
    if t.len() == 2 {
        println!("Yes");
        return;
    }

    let i2 = ind[t[2]].upper_bound(&v1);
    if i2 == ind[t[2]].len() {
        println!("No");
        return;
    }
    let v2 = ind[t[2]][i2];
    if v1 >= v2 {
        println!("No");
        return;
    }
    println!("Yes");
}
