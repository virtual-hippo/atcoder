// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        x: u64,
        k: u64,
    }
    let mut current = x;
    for i in 1..k+1 {
        current = (current as f64 / 10_u64.pow(i as u32) as f64).round() as u64 * 10_u64.pow(i as u32)
    }
    println!("{}", current);
}

