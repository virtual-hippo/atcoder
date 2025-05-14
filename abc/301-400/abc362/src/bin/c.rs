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
        n: usize,
    }
    let mut sum = 0;
    let mut ans = vec![];
    let mut amari = vec![];

    for _ in 0..n {
        input! {
            l: i64,
            r: i64,
        }
        sum += l;
        ans.push(l);
        amari.push(r - l);
    }

    if sum == 0 {
        println!("Yes");
        for i in 0..n {
            if i == n - 1 {
                print!("{}\n", ans[i]);
            } else {
                print!("{} ", ans[i]);
            }
        }
        return;
    }

    if sum > 0 {
        println!("No");
        return;
    }

    for i in 0..n {
        if sum == 0 {
            break;
        }
        let x = amari[i].min(-sum);
        ans[i] += x;
        sum += x;
    }

    if sum != 0 {
        println!("No");
        return;
    }

    println!("Yes");
    for i in 0..n {
        if i == n - 1 {
            print!("{}\n", ans[i]);
        } else {
            print!("{} ", ans[i]);
        }
    }
}
