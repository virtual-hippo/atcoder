#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }
    for i in 0..n - m + 1 {
        for j in 0..n - m + 1 {
            let mut flag = true;

            for ii in 0..m {
                for jj in 0..m {
                    if s[i + ii][j + jj] != t[ii][jj] {
                        flag = false;
                        break;
                    }
                }

                if !flag {
                    break;
                }
            }
            if flag {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
