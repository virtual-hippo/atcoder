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
        k: usize,
    }

    let mut c_a_r = vec![];

    for _ in 0..m {
        input! {
            c: usize,
            a: [usize; c],
            r: char,
        }
        c_a_r.push((c, a, r));
    }

    let mut ans = 0;
    for bit in 0..1 << n {
        let mut keys = vec![false; n];
        for i in 0..n {
            if bit & (1 << i) == 0 {
                keys[i] = true;
            }
        }

        let is_ok = (0..m)
            .map(|i| (i, c_a_r[i].1.iter().filter(|&&v| keys[v - 1]).count()))
            .all(|(i, real_num)| (real_num >= k && c_a_r[i].2 == 'o') || (real_num < k && c_a_r[i].2 == 'x'));

        if is_ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
