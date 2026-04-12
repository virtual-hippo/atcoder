#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{input, marker::*};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::*;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        l: [i64; n],
    }

    let mut ans = 0;
    for bit in 0..(1 << n) {
        let mut cnt: usize = 0;
        let mut now = 0;
        for i in 0..n {
            let pre = now;
            if bit & (1 << i) != 0 {
                now += l[i];
            } else {
                now -= l[i];
            }

            if pre < 0 && 0 <= now || now < 0 && 0 <= pre {
                cnt += 1;
            }
        }

        ans = ans.max(cnt);
    }

    println!("{}", ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
