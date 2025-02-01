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
        w: usize,
    }

    let mut db = FxHashMap::default();

    let xy = {
        let mut ret = Vec::with_capacity(n);
        for i in 0..n {
            input! {
                x: usize,
                y: usize,
            }
            let x = x - 1;
            let y = y - 1;
            ret.push((x, y));
            db.insert((x, y), i);
        }
        ret
    };

    let mut cols = vec![vec![]; w];
    for i in 0..n {
        let (x, y) = xy[i];
        cols[x].push(y);
    }
    for i in 0..w {
        cols[i].sort();
    }
    let max_block = (0..w).map(|i| cols[i].len()).max().unwrap();
    let min_block = (0..w).map(|i| cols[i].len()).min().unwrap();

    let mut ans = vec![usize::MAX; n];

    {
        for i in 0..max_block {
            let mut has_none = false;
            for j in 0..w {
                if i < cols[j].len() {
                } else {
                    has_none = true;
                }
            }
            if has_none {
                continue;
            } else {
                let mut max = 0;
                for j in 0..w {
                    let height = cols[j][i];
                    max = max.max(height);
                }

                for j in 0..w {
                    let (x, y) = (j, cols[j][i]);
                    let v: usize = *db.get(&(x, y)).unwrap();
                    ans[v] = 1 + max;
                }
            }
        }
    }
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            t: usize,
            a: usize,
        }

        let a = a - 1;
        if min_block == 0 {
            println!("Yes");
            continue;
        }

        if ans[a] > t {
            println!("Yes");
            continue;
        } else {
            println!("No");
        }
    }
}
