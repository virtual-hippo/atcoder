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
        s: [String; n],
    }
    let mut authed = false;
    let mut ans = 0;

    for i in 0..n {
        if s[i] == "login" {
            authed = true;
        } else if s[i] == "logout" {
            authed = false;
        } else if s[i] == "private" {
            if !authed {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
