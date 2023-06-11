// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    for i in k..n {
        print!("{} ", a[i]);
    }
    let min = std::cmp::min(k,n);
    for _ in 0..min-1 {
        print!("{} ", 0);
    }
    println!("0");
}

