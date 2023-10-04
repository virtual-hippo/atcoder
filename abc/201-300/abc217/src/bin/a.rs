// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    if s < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
