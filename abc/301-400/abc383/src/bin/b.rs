#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    }
    let yuka_zahyo = {
        let mut ret = vec![];
        for i in 0..h {
            for j in 0..w {
                if s[i][j] == '.' {
                    ret.push((i as i32, j as i32));
                }
            }
        }
        ret
    };

    let n = yuka_zahyo.len();

    let mut ans = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            let pos0 = yuka_zahyo[i];
            let pos1 = yuka_zahyo[j];
            let mut set = HashSet::new();
            for ii in 0..h as i32 {
                for jj in 0..w as i32 {
                    if s[ii as usize][jj as usize] == '#' {
                        continue;
                    }
                    if (pos0.0 - ii).abs() + (pos0.1 - jj).abs() <= d as i32 {
                        set.insert((ii, jj));
                    }
                    if (pos1.0 - ii).abs() + (pos1.1 - jj).abs() <= d as i32 {
                        set.insert((ii, jj));
                    }
                }
            }
            ans = ans.max(set.len());
        }
    }

    println!("{}", ans);
}
