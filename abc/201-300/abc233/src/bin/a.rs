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
    let mut ans = 0;
    while ans * 10 + x < y {
        ans += 1;
    }
    println!("{}", ans);
}

