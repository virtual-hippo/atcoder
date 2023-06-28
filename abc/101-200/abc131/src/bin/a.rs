// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut prev = 'a';
    for ch in s.chars() {
        if ch == prev {
            println!("Bad");
            return
        }
        prev = ch;
    }
    println!("{}", "Good");
}

