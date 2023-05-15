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
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == n {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

