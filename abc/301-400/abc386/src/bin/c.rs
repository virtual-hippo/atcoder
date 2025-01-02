#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        _k: usize,
        mut s: Chars,
        mut t: Chars,
    }
    if s == t {
        println!("Yes");
        return;
    }
    if (s.len() as i64 - t.len() as i64).abs() > 1 {
        println!("No");
        return;
    }
    if s.len() == t.len() {
        let cnt = (0..s.len()).filter(|&i| s[i] != t[i]).count();
        if cnt == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    if s.len() > t.len() {
        t.push('_');
        let mut si = 0;
        let mut ti = 0;
        let mut cnt = 0;
        while si < s.len() {
            if s[si] != t[ti] {
                si += 1;
                cnt += 1;
                continue;
            } else {
                si += 1;
                ti += 1;
            }
        }
        if cnt == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    if s.len() < t.len() {
        s.push('_');
        let mut si = 0;
        let mut ti = 0;
        let mut cnt = 0;
        while ti < t.len() {
            if s[si] != t[ti] {
                ti += 1;
                cnt += 1;
                continue;
            } else {
                si += 1;
                ti += 1;
            }
        }
        if cnt == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }
}
