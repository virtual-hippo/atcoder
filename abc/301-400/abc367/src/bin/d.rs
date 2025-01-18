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
        a: [usize; n],
    }
    let s = {
        let mut s = vec![0; 2 * n + 1];
        for i in 0..2 * n {
            s[i + 1] = s[i] + a[i % n];
            s[i + 1] %= m;
        }
        s
    };
    let mut cnt = vec![0_usize; m];
    let mut ans = 0;
    for i in 0..2 * n {
        if i >= n {
            ans += cnt[s[i - n]].saturating_sub(1);
            cnt[s[i - n]] -= 1;
        }
        cnt[s[i]] += 1;
    }

    println!("{}", ans);
}
