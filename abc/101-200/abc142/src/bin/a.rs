// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let cnt = (1..n+1).fold(0, |sum, x| if x % 2 == 1 {sum+1} else {sum});
    println!("{}", (cnt as f64) / (n as f64));
}

