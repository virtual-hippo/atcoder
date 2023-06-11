// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        t: u64,
    }
    let f = |x: u64| -> u64 {x*x + 2 * x +3};

    println!("{}", f(f(f(t)+t)+f(f(t))));
}

