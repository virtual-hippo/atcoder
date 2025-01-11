#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        m: usize,
    }
    let mut m = m;
    let mut ans = vec![];
    let mut i: u32 = 10;
    loop {
        while m >= 3_usize.pow(i) {
            m -= 3_usize.pow(i);
            ans.push(i);
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}
