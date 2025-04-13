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
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut t = s.clone();

    for i in 0..n {
        if t[i] == '?' {
            if (i > 0 && t[i - 1] == 'o') || (i < n - 1 && t[i + 1] == 'o') {
                t[i] = '.';
            }
        }
    }

    if k == t.iter().filter(|&&c| c == 'o').count() {
        for i in 0..n {
            if t[i] == '?' {
                t[i] = '.';
            }
        }
    } else {
        let m = {
            let mut cloned = t.clone();
            for i in 0..n {
                if cloned[i] == '?'
                    && (i == 0 || cloned[i - 1] != 'o')
                    && (i == n - 1 || cloned[i + 1] != 'o')
                {
                    cloned[i] = 'o';
                }
            }
            cloned.iter().filter(|&&c| c == 'o').count()
        };

        if m == k {
            let mut cnt = 0;
            for i in 0..n {
                if t[i] == '?' {
                    cnt += 1
                } else {
                    if cnt % 2 == 1 {
                        for j in 0..cnt {
                            let k = i - j - 1;
                            if j % 2 == 0 {
                                t[k] = 'o';
                            } else {
                                t[k] = '.';
                            }
                        }
                    }
                    cnt = 0;
                }
            }
            if cnt % 2 == 1 {
                for j in 0..cnt {
                    let k = n - j - 1;
                    if j % 2 == 0 {
                        t[k] = 'o';
                    } else {
                        t[k] = '.';
                    }
                }
            }
        }
    }

    for i in 0..n {
        print!("{}", t[i]);
    }
    println!();
}
