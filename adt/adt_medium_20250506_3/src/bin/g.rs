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
        q: usize,
    }

    let mut diff = VecDeque::new();
    diff.push_back(0);

    for _ in 0..q {
        input! {
            query: usize,
        }

        match query {
            1 => {
                diff.push_back(0);
            },
            2 => {
                input! {
                    t: i64,
                }
                diff[0] += t;
                let tail = diff.len() - 1;
                diff[tail] -= t;
            },
            3 => {
                input! {
                    h: i64,
                }

                let mut ans = 0;
                while diff[0] >= h {
                    diff[1] += diff[0];
                    diff.pop_front();
                    ans += 1;
                }
                println!("{}", ans);
            },
            _ => unreachable!(),
        }
    }
}
