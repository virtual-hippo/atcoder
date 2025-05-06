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
    }
    if n == 0 {
        println!("0");
        return;
    }

    let count = |x: usize| -> usize {
        let mut a = 1;
        let mut res = 0;
        while a * a <= x {
            if x % a != 0 {
                a += 1;
                continue;
            }

            if a * a == x {
                res += 1;
            } else {
                res += 2;
            }
            a += 1;
        }
        res
    };

    let mut ans = 0;
    for ab in 1..n {
        let dc = n - ab;

        let ab_cnt = count(ab);
        let dc_cnt = count(dc);

        ans += dc_cnt * ab_cnt;
    }

    println!("{}", ans);
}
