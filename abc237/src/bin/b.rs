// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (h,w): (usize, usize),
        a: [[usize; w]; h],
    }
    for j in 0..w {
        for i in 0..h {
            print!("{} ", a[i][j]);
        }
        println!("");
    }
}

