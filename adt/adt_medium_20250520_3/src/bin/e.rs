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
        _k: usize,
        s: Chars,
        t: Chars,
    }

    if s == t {
        println!("Yes");
        return;
    }

    if s.len() == t.len() {
        let dif_cnt = (0..s.len()).filter(|&i| s[i] != t[i]).count();
        if dif_cnt == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    if s.len() + 1 == t.len() {
        let mut i = 0;
        let mut j = 0;

        while i < s.len() {
            if s[i] != t[j] {
                j += 1;

                if j - i > 1 {
                    println!("No");
                    return;
                }
            }

            if s[i] != t[j] {
                println!("No");
                return;
            }

            i += 1;
            j += 1;
        }
        println!("Yes");
        return;
    }

    if s.len() - 1 == t.len() {
        let mut i = 0;
        let mut j = 0;

        while j < t.len() {
            if s[i] != t[j] {
                i += 1;
                if i - j > 1 {
                    println!("No");
                    return;
                }
            }
            if s[i] != t[j] {
                println!("No");
                return;
            }

            i += 1;
            j += 1;
        }
        println!("Yes");
        return;
    }

    println!("No");
}
