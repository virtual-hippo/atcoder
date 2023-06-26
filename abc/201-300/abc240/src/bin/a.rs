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
    }
    if a == 1 && b == 10 {
        println!("Yes");
    } else if b - a== 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}

