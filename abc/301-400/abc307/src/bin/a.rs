// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 7 * n],
    }
    for i in 0..n {
        let mut current = 0;
        for j in 0..7 {
            current += a[7 * i + j]
        }
        print!("{} ", current);
    }
}

