// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut max = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            let current = (xy[j].0 - xy[i].0) *
            (xy[j].0 - xy[i].0) +
            (xy[j].1 - xy[i].1) *
            (xy[j].1 - xy[i].1);
            max = std::cmp::max(max, current);

        }
    }
    println!("{}", (max as f64).sqrt());
}

