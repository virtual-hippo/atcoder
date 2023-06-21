// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut min_vec = vec![0; n];
    min_vec[0] = p[0];
    for i in 1..n {
        min_vec[i] = std::cmp::min(min_vec[i-1], p[i]);
    }
    let mut cnt = 0;
    for i in 0..n {
        if min_vec[i] >= p[i] {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

