#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = 1;

    for w in 1..n {
        for si in 0..w {
            let mut pre = h[si];
            let mut len = 0;
            for i in (si..n).step_by(w) {
                if pre == h[i] {
                    len += 1;
                } else {
                    pre = h[i];
                    len = 1;
                }
                ans = ans.max(len);
            }
        }
    }

    println!("{}", ans);
}
