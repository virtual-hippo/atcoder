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
        s: String,
    }
    let mut stack = vec![];
    for c in s.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == '[' {
            stack.push(c);
        } else if c == '<' {
            stack.push(c);
        } else if c == ')' {
            if stack.last() == Some(&'(') {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else if c == ']' {
            if stack.last() == Some(&'[') {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else if c == '>' {
            if stack.last() == Some(&'<') {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
    }

    if stack.len() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
