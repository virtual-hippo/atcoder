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
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],

    }

    if a == b {
        println!("Yes");
        return;
    }

    for i in 0..(1 << h1) {
        let mut row = vec![];
        for k in 0..h1 {
            if (i >> k) & 1 == 1 {
                row.push(k)
            }
        }

        if row.len() != h2 {
            continue;
        }

        for j in 0..(1 << w1) {
            let mut col = vec![];
            for k in 0..w1 {
                if (j >> k) & 1 == 1 {
                    col.push(k)
                }
            }

            if col.len() != w2 {
                continue;
            }

            {
                let mut ok = true;
                for (i, ii) in row.iter().enumerate() {
                    for (j, jj) in col.iter().enumerate() {
                        if a[*ii][*jj] != b[i][j] {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
