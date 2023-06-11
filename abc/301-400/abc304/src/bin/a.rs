// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        sa: [(String, usize); n],
    }
    let mut min = 1_000_000_000_000;
    let mut ind = 0;
    for i in 0..n {
        if min >  sa[i].1 {
            min = sa[i].1;
            ind = i;
        }
    }
    for i in ind..n {
        println!("{}", sa[i].0);
    }
    for i in 0..ind {
        println!("{}", sa[i].0);
    }
}

