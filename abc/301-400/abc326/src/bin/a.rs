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
    if x < y {
        if y - x < 3 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if x - y < 4 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
