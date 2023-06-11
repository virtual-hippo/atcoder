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
    let mut count = 0;
    for _ in 0..n {
        input! {
            d: (usize, usize),
        }
        if d.0 == d.1 {
            count += 1;
        } else {
            count = 0;
        }
        if count == 3 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

