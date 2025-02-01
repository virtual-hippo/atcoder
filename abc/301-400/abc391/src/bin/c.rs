#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashSet, VecDeque};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // hato がどこにイルカ
    let mut hato = (0..n).collect_vec();

    // 何匹か
    let mut su = vec![1; n];

    let mut set = HashSet::with_capacity(n);

    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    p: usize,
                    h: usize,
                }
                let p = p - 1;
                let h = h - 1;

                su[hato[p]] -= 1;
                su[h] += 1;

                if su[hato[p]] < 2 {
                    set.remove(&hato[p]);
                }
                if su[h] > 1 {
                    set.insert(h);
                }

                hato[p] = h;
            }
            2 => {
                println!("{}", set.len());
            }
            _ => unreachable!(),
        }
    }
}
