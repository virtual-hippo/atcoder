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
        c: usize,
    }
    for i in a..=b {
        if i % c == 0 {
            println!("{}", i);
            return;
        }
    }
    let ans = -1;
    println!("{}", ans);
}

