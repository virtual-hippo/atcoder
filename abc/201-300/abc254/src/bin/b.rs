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
    let mut prev = vec![];
    for i in 0..n {
        let len = i+1;
        let mut current = vec![1; len];
        for j in 0..len {
            if j == 0 || j == i {
                current[j] = 1;
                print!("{} ",current[j]);
            } else {
                current[j] = prev[j-1] + prev[j];
                print!("{} ",current[j]);
            }
        }
        prev = current;
        println!("");
    }
}

