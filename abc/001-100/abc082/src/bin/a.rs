// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if (a + b) % 2 == 0 {
        println!("{}", (a+b)/2);
    } else {
        println!("{}", (a+b)/2 + 1);
    }
}

