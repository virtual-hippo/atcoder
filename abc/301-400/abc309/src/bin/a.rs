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
    if a + 1 == b && a %3 != 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

