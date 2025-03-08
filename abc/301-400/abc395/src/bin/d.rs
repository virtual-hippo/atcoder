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
        q: usize,
    }

    let mut p_to_box = (0..n).collect_vec();
    let mut box_to_label = (0..n).collect_vec();
    let mut label_to_box = (0..n).collect_vec();

    for _ in 0..q {
        input! {
            query: usize,
        }

        match query {
            1 => {
                input! {
                    a: usize,
                    b: usize,
                }
                let a = a - 1;
                let b = b - 1;
                p_to_box[a] = label_to_box[b];
            }
            2 => {
                input! {
                    a: usize,
                    b: usize,
                }
                let a = a - 1;
                let b = b - 1;
                (label_to_box[a], label_to_box[b]) = (label_to_box[b], label_to_box[a]);
                (box_to_label[label_to_box[a]], box_to_label[label_to_box[b]]) =
                    (box_to_label[label_to_box[b]], box_to_label[label_to_box[a]]);
            }
            3 => {
                input! {
                    a: usize,
                }
                let a = a - 1;
                println!("{}", box_to_label[p_to_box[a]] + 1);
            }
            _ => unreachable!(),
        }
    }
}
