#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, usize};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: [Chars; 3],
    }
    for i in 0..2 * n {
        for j in 0..3 {
            let ch = s[j][i % n];
            s[j].push(ch);
        }
    }

    let init = usize::MAX;
    let mut ans = init;

    for vec in [0, 1, 2].iter().permutations(3) {
        for target in 0..10 {
            let mut pi = 0;
            for i in 0..n * 3 {
                let hito = *vec[pi];
                if s[hito][i] as u8 - b'0' == target {
                    pi += 1;
                }
                if pi > 2 {
                    ans = ans.min(i);
                    break;
                }
            }
        }
    }

    if ans == init {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
