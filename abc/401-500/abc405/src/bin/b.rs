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
        m: usize,
        mut a: [usize; n],
    }

    let mut cnt = vec![0; m + 1];
    cnt[0] = 1;
    for v in a.iter() {
        cnt[*v] += 1;
    }

    let is_ok = |lis: &Vec<usize>| lis.iter().all(|&x| x >= 1);

    let mut ans = 0;
    while is_ok(&cnt) {
        let v = a.pop().unwrap();
        cnt[v] -= 1;
        ans += 1;
    }

    println!("{}", ans);
}
