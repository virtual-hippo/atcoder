// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        b: i64,
    }
    if (x-a).abs() < (x-b).abs() {
        println!("A");
    } else {
        println!("B");
    }
}

