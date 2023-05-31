// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        mut y: usize,
    }
    while y % 4 != 2 {
        y += 1;
    }
    println!("{}", y);
}

