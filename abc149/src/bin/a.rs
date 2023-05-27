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
    for i in 0..t.len() {
        print!("{}", t[i]);
    }
    for i in 0..s.len() {
        print!("{}", s[i]);
    }
}

