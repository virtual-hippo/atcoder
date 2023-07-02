// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        x: usize,
        a: usize,
    }
    if x < a {
        println!("0");
    } else {
        println!("10");
    }
}

