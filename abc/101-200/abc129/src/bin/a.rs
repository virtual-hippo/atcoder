// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        p: usize,
        q: usize,
        r: usize,
    }
    let ans = std::cmp::min(std::cmp::min(p+q, q+r), r+p);
    println!("{}", ans);
}

