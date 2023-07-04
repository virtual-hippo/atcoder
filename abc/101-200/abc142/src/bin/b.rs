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
        h: [usize; n],
    }
    println!("{}", h.iter().filter(|&&x| x >= k).count());
}

