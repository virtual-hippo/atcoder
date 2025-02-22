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
        c: [Chars;n],
    }

    let mut ans = vec![vec![usize::MAX; n]; n];

    let mut que = VecDeque::new();
    for i in 0..n {
        ans[i][i] = 0;
        que.push_back((i, i));
    }

    itertools::iproduct!(0..n, 0..n).for_each(|(i, j)| {
        if c[i][j] != '-' && i != j {
            ans[i][j] = 1;
            que.push_back((i, j));
        }
    });

    while let Some((i, j)) = que.pop_front() {
        for k in 0..n {
            for l in 0..n {
                if c[k][i] != '-' && c[j][l] != '-' && c[k][i] == c[j][l] && ans[k][l] == usize::MAX
                {
                    ans[k][l] = ans[i][j] + 2;
                    que.push_back((k, l));
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if ans[i][j] != usize::MAX {
                print!("{} ", ans[i][j]);
            } else {
                print!("-1 ");
            }
        }
        println!();
    }
}
