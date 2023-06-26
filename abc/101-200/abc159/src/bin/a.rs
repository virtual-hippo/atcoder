// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let a = if n > 1 {
        n * (n-1) / 2
    } else {
        0
    };
    let b = if m > 1 {
        m * (m-1) / 2
    } else {
        0
    };
    
    println!("{}", a+b);
}

