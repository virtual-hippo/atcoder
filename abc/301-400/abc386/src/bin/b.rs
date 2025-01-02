#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] != '0' {
            ans += 1;
        } else {
            if i == s.len() - 1 {
                ans += 1;
            } else if s[i + 1] == '0' {
                i += 2;
                ans += 1;
                continue;
            } else {
                ans += 1;
            }
        }
        i += 1;
    }
    println!("{}", ans);
}
