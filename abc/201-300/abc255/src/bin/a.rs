// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[usize; 2]; 2],
    }
    println!("{}", a[r-1][c-1]);
}

