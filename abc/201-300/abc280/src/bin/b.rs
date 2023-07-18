// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    }
    let mut a = Vec::with_capacity(n);
    a.push(s[0]);
    for i in 1..n {
        a.push(s[i] - s[i-1])
    }
    for i in 0..n {
        print!("{} ", a[i]);
    }
}

