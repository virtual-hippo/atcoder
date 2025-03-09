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
        b: [i64; n],
        w: [i64; m],
    }
    let b = b
        .iter()
        .copied()
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<_>>();
    let w = w
        .iter()
        .copied()
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<_>>();

    let mut ans = 0;
    let mut now = 0;
    for i in 0..n {
        if i >= m {
            if b[i] < 0 {
                break;
            } else {
                ans += b[i];
                continue;
            }
        } else {
            let v = if w[i] < 0 { b[i] } else { b[i] + w[i] };
            now += v;
            ans = ans.max(now);
        }
    }

    println!("{}", ans);
}
