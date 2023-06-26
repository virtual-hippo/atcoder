// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(a[i]);
    }
    println!("{}", set.len());
}

