// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut s = vec![0; n+1];
    for i in 0..n {
        s[i+1] = s[i] + a[i];
    }
    println!("Yes");
}

