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
    let x = n % 5;
    if x < 3 {
        println!("{}", n-x);
    } else {
        println!("{}", n+(5-x));
    }
}

