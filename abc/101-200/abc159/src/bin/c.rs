// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        l: usize,
    }
    let l_f = (l as f64) /3.0;
    println!("{}", (l_f * l_f * l_f));
}

