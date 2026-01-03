#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64;n],
        b: [i64;m],
    }

    for i in 0..n - 1 {
        a[i + 1] = a[i].min(a[i + 1]);
    }

    for i in 0..m {
        let is_ok = |ai: usize| a[ai] > b[i];
        let (_, ans) = binary_search((-1, n as i64), is_ok);
        if ans == n {
            println!("{}", -1);
        } else {
            println!("{}", ans + 1);
        }
    }
}

pub fn binary_search<F: Fn(usize) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (usize, usize) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(mid as usize) {
            left = mid;
        } else {
            right = mid;
        }
    }
    (left as usize, right as usize)
}

fn solve2() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut ans = vec![-1; m];
    let b = b
        .iter()
        .copied()
        .enumerate()
        .sorted_by_key(|&(i, v)| (std::cmp::Reverse(v), i))
        .collect_vec();

    let mut i = 0;
    for j in 0..m {
        while i < n && b[j].1 < a[i] {
            i += 1;
        }
        if i == n {
            break;
        }
        ans[b[j].0] = i as i64 + 1;
    }

    for j in 0..m {
        println!("{}", ans[j]);
    }
}
