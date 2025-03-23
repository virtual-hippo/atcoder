#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut heap = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();
    for i in 0..n {
        heap.push((Reverse(i), Reverse(0)));
    }

    let mut somen = vec![0; n];

    for _ in 0..m {
        input! {
            t: usize,
            w: usize,
            s: usize,
        }

        {
            let mut j = 0;
            let len = heap2.len();
            while j < len {
                if let Some((Reverse(tt), Reverse(i))) = heap2.pop() {
                    if tt <= t {
                        heap.push((Reverse(i), Reverse(tt)));
                    } else {
                        heap2.push((Reverse(tt), Reverse(i)));
                        break;
                    }
                }
                j += 1;
            }
        }

        if let Some((Reverse(i), _)) = heap.pop() {
            somen[i] += w;
            heap2.push((Reverse(t + s), Reverse(i)));
        }
    }
    for i in 0..n {
        println!("{}", somen[i]);
    }
}
