// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut table = vec![0; n];
    for i in 0..n{
        input! {
            a: usize,
        }
        table[a-1] = i+1;
    }
    for i in 0..n-1{
        print!("{} ", table[i]);
    }
    print!("{}", table[n-1]);
}

