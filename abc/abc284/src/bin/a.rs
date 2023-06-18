// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    for i in (0..n).rev() {
        println!("{}", s[i]);
    }
}

