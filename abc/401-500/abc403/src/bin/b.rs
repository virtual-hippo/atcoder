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
        t: Chars,
        u: Chars,
    }

    for i in 0..t.len() - u.len() + 1 {
        let mut cnt = 0;
        let mut flag = true;
        for j in 0..u.len() {
            if t[i + j] == u[j] {
                continue;
            }
            if cnt < 4 && t[i + j] == '?' {
                cnt += 1;
                continue;
            }
            if t[i + j] != u[j] {
                flag = false;
            }
        }
        if flag {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
