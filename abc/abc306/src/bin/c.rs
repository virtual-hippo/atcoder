// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 3 * n],
    }
    let mut vec = vec![vec![]; n+1];
    for i in 0..3*n {
        vec[a[i]].push(i+1);
    }
    let mut vec2 = vec![];
    for i in 1..=n {
        vec2.push((i, vec[i][1]));
    }
    vec2.sort_by(|a, b| a.1.cmp(&b.1));
    for (i, _) in vec2.iter() {
        print!("{} ", i);
    }
}

