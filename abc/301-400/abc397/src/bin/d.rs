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
        n: u64,
    }

    let mut a = 1;
    while a * a * a <= n {
        if n % a != 0 {
            a += 1;
            continue;
        }

        let b = n / a;
        let is_ok = |y: u64| {
            let x = a + y;
            (x * x + x * y + y * y) <= b
        };
        let y = binary_search((1, 1_000_000_000), is_ok).0;
        let x = a + y;
        if (x * x + x * y + y * y) == b {
            println!("{} {}", x, y);
            return;
        }

        a += 1;
    }

    println!("{}", -1);
}

pub fn binary_search<F: Fn(u64) -> bool>(initial_pos: (u64, u64), is_ok: F) -> (u64, u64) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while left + 1 < right {
        let mid = (left + right) / 2;
        if is_ok(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    (left, right)
}
