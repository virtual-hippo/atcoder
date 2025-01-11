#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
    }

    let mut cnt = 0;
    for i in 0..n - 2 {
        if format!("{}{}{}", s[i], s[i + 1], s[i + 2]) == "ABC" {
            cnt += 1;
        }
    }

    for _ in 0..q {
        input! {
            x: usize,
            c: char,
        }
        let x = x - 1;
        if x < n - 2 && s[x] == 'A' && s[x + 1] == 'B' && s[x + 2] == 'C' && c != 'A' {
            cnt -= 1;
        }
        if 0 < x && x < n - 1 && s[x - 1] == 'A' && s[x] == 'B' && s[x + 1] == 'C' && c != 'B' {
            cnt -= 1;
        }
        if 1 < x && s[x - 2] == 'A' && s[x - 1] == 'B' && s[x] == 'C' && c != 'C' {
            cnt -= 1;
        }

        if x < n - 2 && s[x] != 'A' && s[x + 1] == 'B' && s[x + 2] == 'C' && c == 'A' {
            cnt += 1;
        }
        if 0 < x && x < n - 1 && s[x - 1] == 'A' && s[x] != 'B' && s[x + 1] == 'C' && c == 'B' {
            cnt += 1;
        }
        if 1 < x && s[x - 2] == 'A' && s[x - 1] == 'B' && s[x] != 'C' && c == 'C' {
            cnt += 1;
        }
        s[x] = c;
        println!("{}", cnt);
    }
}
