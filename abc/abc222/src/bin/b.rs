// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }
    let ans: Vec<&usize> = a.iter().filter(|x| **x < p).collect();
    println!("{}", ans.len());
}

