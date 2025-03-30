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
        p: [usize; n],
    }
    let mut r = 1;
    let mut kakutei = vec![false; n];
    let mut juni = vec![0; n];
    loop {
        let mut mx = 0;
        let mut ii = vec![];
        for i in (0..n).filter(|&i| kakutei[i] == false) {
            if p[i] > mx {
                mx = p[i];
                ii = vec![i];
            } else if p[i] == mx {
                ii.push(i);
            }
        }
        for &i in ii.iter() {
            kakutei[i] = true;
            juni[i] = r;
        }
        r += ii.len();

        if kakutei.iter().all(|v| *v) {
            break;
        }
    }

    for i in 0..n {
        println!("{}", juni[i]);
    }
}
