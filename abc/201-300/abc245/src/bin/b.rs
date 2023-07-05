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
    let set: HashSet<usize> = a.into_iter().collect();
    for i in 0..2001 {
        if set.contains(&i) == false {
            println!("{}", i);
            return;
        }
    }
}

