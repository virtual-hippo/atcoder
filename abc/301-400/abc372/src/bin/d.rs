#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::{
    collections::{HashMap, HashSet},
    usize,
};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut stack = vec![];
    let mut ans = vec![0; n];

    for i in (0..n).rev() {
        ans[i] = stack.len();

        while !stack.is_empty() && stack[stack.len() - 1] < h[i] {
            stack.pop();
        }
        stack.push(h[i]);
    }

    for i in 0..n {
        print!("{} ", ans[i]);
    }
}
