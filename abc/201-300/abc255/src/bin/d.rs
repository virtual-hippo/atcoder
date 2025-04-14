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
        q: usize,
        a: [usize; n],
    }
    let a = a.into_iter().sorted().collect_vec();
    let s = {
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + a[i];
        }
        s
    };

    for _ in 0..q {
        input! {
            x: usize,
        }
        let l = a.lower_bound(&x);
        let r = a.upper_bound(&x);
        if l == n {
            let ans = x * n - s[n];
            println!("{}", ans);
        } else if l == 0 {
            let ans = if a[0] == x {
                s[n] - s[r] - x * (n - r)
            } else {
                s[n] - x * (n - r)
            };
            println!("{}", ans);
        } else {
            let v0 = x * l - (s[l] - s[0]);
            let v1 = (s[n] - s[r]) - x * (n - r);

            let ans = v0 + v1;
            println!("{}", ans);
        }
    }
}
