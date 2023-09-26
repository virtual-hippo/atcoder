// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    }
    if 3 * x <= y {
        println!("{}", x * n);
    } else {
        println!("{}", (x * (n % 3)) + (y * (n / 3)));
    }
}
