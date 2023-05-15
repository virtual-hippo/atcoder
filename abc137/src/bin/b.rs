// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (k,x): (i32, i32),
    }
    let min = x-k+1;
    let max = std::cmp::min(1000000, x+k-1);
    for i in min..max {
        print!("{} ", i);
    }
    print!("{}", max);
}

