// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }
    ab.sort_by(|(_,a), (_,b)| a.cmp(b));
    let mut current_time = 0;
    for (a, b) in ab.iter() {
        current_time += *a;
        if current_time > *b {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

