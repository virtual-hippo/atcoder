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
        m: usize,
        a: [usize; m],
        s: [Chars; n],
    }

    let mut score = (0..n).collect_vec();
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == 'o' {
                score[i] += a[j];
            }
        }
    }
    let (_, top_score) = score
        .iter()
        .enumerate()
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .map(|(i, &v)| (i, v))
        .collect::<Vec<(usize, usize)>>()[0];

    let sorted_a = a
        .iter()
        .enumerate()
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    for i in 0..n {
        let mut now = score[i];
        let mut j = 0;
        let mut ans = 0;
        while now < top_score {
            if s[i][sorted_a[j]] == 'x' {
                now += a[sorted_a[j]];
                ans += 1;
            }
            j += 1;
        }
        println!("{}", ans);
    }
}
