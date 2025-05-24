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
        h: usize,
        w: usize,
        n: usize,
    }
    let mut row = vec![HashSet::new(); h];
    let mut col = vec![HashSet::new(); w];

    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
        }
        let x = x - 1;
        let y = y - 1;
        row[x].insert(y);
        col[y].insert(x);
    }

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: usize,
                }
                let x = x - 1;
                println!("{}", row[x].len());

                for y in row[x].iter() {
                    col[*y].remove(&x);
                }
                row[x] = HashSet::new();
            },
            2 => {
                input! {
                    y: usize,
                }
                let y = y - 1;
                println!("{}", col[y].len());
                for x in col[y].iter() {
                    row[*x].remove(&y);
                }
                col[y] = HashSet::new();
            },
            _ => unreachable!(),
        }
    }
}
