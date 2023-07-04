// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let even = (0..n).filter(|&x| (x +1) %2 == 1).count();
    println!("{}", even as f64 / n as f64);
}

