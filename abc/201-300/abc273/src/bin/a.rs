// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn f(k: usize) -> usize {
    if k == 0 {
        1
    } else {
        k * f(k-1)
    }
}

fn main() {
    input! {
        n: usize,
    }
    let ans = f(n);
    println!("{}", ans);
}

