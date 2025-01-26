#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

fn main() {
    input! {
        a: [usize; 5],
    }

    for i in 0..4 {
        let mut aa = a.clone();
        let tmp = a[i];
        aa[i] = aa[i + 1];
        aa[i + 1] = tmp;
        let flag = (0..4).filter(|&i| aa[i] > aa[i + 1]).count() == 0;
        if flag {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
