#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, fmt::format};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }
    f(n, k, &r, &mut vec![]);
}

fn f(n: usize, k: usize, r: &Vec<usize>, now: &mut Vec<usize>) {
    let len = now.len();

    if len == n {
        let s: usize = now.iter().sum();
        if s % k == 0 {
            let ans = now.iter().map(|v| v.to_string()).join(" ");
            println!("{}", ans);
        }
        return;
    }

    for i in 1..=r[len] {
        now.push(i);
        f(n, k, r, now);
        now.pop();
    }
}
