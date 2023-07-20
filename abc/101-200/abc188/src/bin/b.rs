// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let mut naiseki = 0;
    for i in 0..n {
        naiseki += a[i] * b[i];
    }
    if naiseki == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

