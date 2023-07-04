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
        c: usize,
    }
    if a <= b && b <= c {
        println!("Yes");
    } else if b <= b && b <= a {
        println!("Yes");
    } else {
        println!("No");
    }
}

