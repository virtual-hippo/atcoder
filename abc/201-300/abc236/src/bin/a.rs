// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        a: usize, 
        b: usize, 
    }
    for i in 0..s.len()  {
        if i == a - 1 {
            print!("{}", s[b-1]);
        } else if i == b - 1 {
            print!("{}", s[a-1]);
        } else {
            print!("{}", s[i]);
        }
    }
}

