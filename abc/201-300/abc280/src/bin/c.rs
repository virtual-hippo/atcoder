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
        t: Chars,
    }
    for i in 0..s.len() {
        if s[i] != t[i] {
            println!("{}", i+1);
            return;
        }
    }
    println!("{}", t.len());
}

