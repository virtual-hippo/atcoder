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
        k: usize,
    }
    if k > a+ b{
        println!("0 0");
    } else if a < k {
        println!("0 {}", b - (k-a));
    } else {
        println!("{} {}", a - k, b);
    }
}

