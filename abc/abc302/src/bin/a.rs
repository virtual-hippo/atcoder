// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }
    if a % b == 0 {
        println!("{}", a/b);
    } else {
        println!("{}", a/b + 1);
    }
}

