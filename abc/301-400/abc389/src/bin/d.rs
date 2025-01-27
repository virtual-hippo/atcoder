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
        r: usize,
    }
    let mut ans = 1 + (r-1) *4;
    let added = {
        let mut ret = 0;
    
        for x in 1..=r {
            
            let is_ok = |y: usize| {
                let x = x as f64 + 0.5;
                let y = y as f64 + 0.5;
                let r = r as f64;
                x * x + y * y <= r * r
            };

            let (limit, _) = binary_search((0,r as i64), is_ok);
            ret += limit as usize;
        }
            
        ret * 4
    };
    ans += added;

    println!("{}", ans);
}

pub fn binary_search<F: Fn(usize) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (i64, i64) {
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
    (left, right)
}

