// 解説AC
#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, usize};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let g = {
        let mut ret = vec![vec![]; n];
        for _ in 0..m {
            input! {
                x: usize,
                y: usize,
                z: usize,
            }
            let x = x - 1;
            let y = y - 1;
            ret[x].push((y, z));
            ret[y].push((x, z));
        }
        ret
    };

    let mut a = vec![0; n];
    let mut is_ok = true;

    // 桁ごとに処理する
    // 10 ** 9 < 2 **30
    for k in 0..30 {
        let mut colors = vec![usize::MAX; n];
        for sv in 0..n {
            if colors[sv] != usize::MAX {
                continue;
            }
            let mut vs = vec![vec![]; 2];

            let mut q = VecDeque::new();
            q.push_back((sv, 0));

            while let Some((v, color)) = q.pop_front() {
                if colors[v] != usize::MAX {
                    if colors[v] != color {
                        is_ok = false;
                    }
                    continue;
                }

                colors[v] = color;
                vs[color].push(v);

                for &(nv, z) in g[v].iter() {
                    q.push_back((nv, color ^ ((z >> k) & 1)));
                }
            }
            // 1 の方を小さくする
            let ind = if vs[0].len() < vs[1].len() { 0 } else { 1 };
            for i in vs[ind].iter() {
                a[*i] |= 1 << k;
            }
        }
    }

    if is_ok {
        let ans = a.iter().join(" ");
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
