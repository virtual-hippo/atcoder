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
        q: usize,
    }

    let mut que = vec![];
    let mut s = vec![0];
    let mut ll = 0;
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    l: usize,
                }
                que.push(l);
                let val = s[s.len()-1];
                s.push(val + l);
            },
            2 => {
                ll +=1;
            },
            3 => {
                input! {
                    k: usize,
                }
                let ans = s[ll+k-1] - s[ll];
                println!("{}", ans);
            },
            _ => unreachable!(),
        }

    }

}

