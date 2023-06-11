// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: [usize; 5],
    }
    let mut set = HashSet::new();
    for i in 0..5 {
        set.insert(n[i]);
    }
    println!("{}", set.len());
}

