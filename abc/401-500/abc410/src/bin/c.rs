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
        q: usize,
    }

    let mut offset = 0;
    let mut a = (0..n).map(|i| i + 1).collect::<Vec<usize>>();

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    p: usize,
                    x: usize,
                }
                let p = p - 1;
                let p = (p + offset) % n;
                a[p] = x;
            },
            2 => {
                input! {
                    p: usize,
                }
                let p = p - 1;
                let p = (p + offset) % n;
                println!("{}", a[p]);
            },
            3 => {
                input! {
                    k: usize,
                }
                offset += k;
                offset %= n;
            },
            _ => unreachable!(),
        }
    }
}
