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
        c: [[usize; 3]; 3],
    }

    let p = iproduct!(0..3, 0..3).collect::<Vec<(usize, usize)>>();
    let mut bunbo = 0;
    let mut bunshi = 0;
    for perm in p.iter().permutations(9) {
        let mut rows = vec![vec![]; 3];
        let mut cols = vec![vec![]; 3];
        let mut dig = vec![vec![]; 2];

        let mut is_ok = true;
        for &&(i, j) in perm.iter() {
            c[i][j];
            rows[i].push(c[i][j]);
            cols[j].push(c[i][j]);
            if i == j {
                dig[0].push(c[i][j]);
            }
            if i + j == 2 {
                dig[1].push(c[i][j]);
            }

            if rows[i].len() == 3 && rows[i][0] == rows[i][1] && rows[i][1] != rows[i][2] {
                is_ok = false;
            }

            if cols[j].len() == 3 && cols[j][0] == cols[j][1] && cols[j][1] != cols[j][2] {
                is_ok = false;
            }

            if dig[0].len() == 3 && dig[0][0] == dig[0][1] && dig[0][1] != dig[0][2] {
                is_ok = false;
            }

            if dig[1].len() == 3 && dig[1][0] == dig[1][1] && dig[1][1] != dig[1][2] {
                is_ok = false;
            }
        }
        if is_ok {
            bunshi += 1;
        }
        bunbo += 1;
    }

    let ans = bunshi as f64 / bunbo as f64;
    println!("{}", ans);
}
