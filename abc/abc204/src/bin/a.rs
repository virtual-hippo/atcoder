// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    if x != y {
        for i in 0..3 {
            if i != x && i != y {
                println!("{}", i);
            }
        }
    } else {
        println!("{}", x);
    }
}

