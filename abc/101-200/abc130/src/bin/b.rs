// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        l: [usize; n],
    }
    let mut d = vec![0; n+1];
    for i in 0..n {
        d[i+1] = d[i] + l[i];
    }
    let ans = d.iter().filter(|&&v| v <= x).count();
    println!("{}", ans);
}

