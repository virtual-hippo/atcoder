// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    }
    for i in 0..n {
        if p[i] == x {
            println!("{}", i+1);
            return;
        }
    }
}

