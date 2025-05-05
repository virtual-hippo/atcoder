#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, VecDeque};
use superslice::Ext;

fn _dfs(visit_count: &mut Vec<usize>, table: &Vec<Vec<usize>>, c: &Vec<u64>, n: usize, m: usize, ans: &mut u64) {
    if visit_count.len() == n {
        let mut counter = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                counter[j] += table[i][j] * visit_count[i];
            }
        }

        if counter.iter().all(|&x| x >= 2) {
            let mut sum = 0;
            for i in 0..n {
                sum += visit_count[i] as u64 * c[i];
            }
            *ans = (*ans).min(sum);
        }
        return;
    }

    for i in 0..3 {
        visit_count.push(i);
        _dfs(visit_count, table, c, n, m, ans);
        visit_count.pop();
    }
}

/// 再帰による全探索
fn _solve1() {
    input! {
        n: usize,
        m: usize,
        c: [u64; n],
    }

    let mut table = vec![vec![0; m]; n];

    for i in 0..m {
        input! {
            k: usize,
        }
        for _ in 0..k {
            input! {
                a: usize,
            }
            let a = a - 1;
            table[a][i] = 1;
        }
    }

    let mut visit_count = vec![];
    let mut ans = u64::MAX;
    _dfs(&mut visit_count, &table, &c, n, m, &mut ans);
    println!("{}", ans);
}

/// 3 進法を利用した全探索
fn _solve2() {
    input! {
        n: usize,
        m: usize,
        c: [u64; n],
    }

    let table = {
        let mut table = vec![vec![0; m]; n];

        for i in 0..m {
            input! {
                k: usize,
            }
            for _ in 0..k {
                input! {
                    a: usize,
                }
                let a = a - 1;
                table[a][i] = 1;
            }
        }
        table
    };

    let p3 = (0..n).fold(vec![1_u64], |mut acc, i| {
        let v = acc[i] * 3;
        acc.push(v);
        acc
    });

    let mut ans = u64::MAX;

    for s in 0..p3[n] {
        let visit_count = (0..n).map(|i| s / p3[i] % 3).collect_vec();

        let mut counter = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                counter[j] += table[i][j] * visit_count[i];
            }
        }

        if counter.iter().all(|&x| x >= 2) {
            let mut sum = 0;
            for i in 0..n {
                sum += visit_count[i] * c[i];
            }
            ans = ans.min(sum);
        }
    }

    println!("{}", ans);
}

#[fastout]
fn main() {
    _solve2();
}
