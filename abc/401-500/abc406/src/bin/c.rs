#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet, VecDeque};
use superslice::Ext;

// 本番で用いた解法
fn _solve() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];

    for i in 0..n - 2 {
        if i < n - 3 && p[i] < p[i + 1] {
            a.push(i);
        }

        if p[i] < p[i + 1] && p[i + 1] > p[i + 2] {
            b.push(i);
        }
        if p[i] > p[i + 1] && p[i + 1] < p[i + 2] {
            c.push(i);
        }
    }

    if a.len() == 0 || b.len() == 0 || c.len() == 0 {
        println!("0");
        return;
    }

    let mut pb = 0;
    let mut pc = 0;

    let mut ans = 0;
    for i in 0..a.len() {
        while pb < b.len() && a[i] > b[pb] {
            pb += 1;
        }
        while pc < c.len() && a[i] > c[pc] {
            pc += 1;
        }

        if pb == b.len() || pc == c.len() {
            break;
        }

        if b[pb] < c[pc] && (pb == b.len() - 1 || c[pc] < b[pb + 1]) {
            let tail = c[pc] + 2;
            let b_next = if pb == b.len() - 1 { n } else { b[pb + 1] + 2 };
            let c_next = if pc == c.len() - 1 { n } else { c[pc + 1] + 2 };
            ans += c_next.min(b_next) - tail;
            continue;
        }

        if c[pc] < b[pb] && (pc == c.len() - 1 || b[pb] < c[pc + 1]) {
            let tail = b[pb] + 2;
            let b_next = if pb == b.len() - 1 { n } else { b[pb + 1] + 2 };
            let c_next = if pc == c.len() - 1 { n } else { c[pc + 1] + 2 };
            ans += c_next.min(b_next) - tail;
            continue;
        }
    }

    println!("{}", ans);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let run_length = (0..n - 1)
        .map(|i| p[i] < p[i + 1])
        .map(|v| if v { '<' } else { '>' })
        .dedup_with_count()
        .collect_vec();

    let mut ans = 0;
    for i in 2..run_length.len() {
        if run_length[i].1 == '<' {
            ans += run_length[i].0 * run_length[i - 2].0;
        }
    }

    println!("{}", ans);
}
