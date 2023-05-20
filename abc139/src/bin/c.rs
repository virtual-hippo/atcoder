// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }
    let mut vec = vec![];
    let mut current = 0;
    let mut prev = 0;
    for i in 0..n {
        if h[i] > prev {
            vec.push(current);
            current = 0;
        } else {
            current += 1;
        }
        prev = h[i];
    }
    vec.push(current);
    let ans = vec.iter().fold(0, |max, x| std::cmp::max(max, *x));
    println!("{}", ans);
}

