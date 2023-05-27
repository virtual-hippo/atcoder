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
    }
    for i in (0..s.len()).rev() {
        if s[i] == '9' {
            print!("{}", 6);
        } else if s[i] == '6' {
            print!("{}", 9);
        } else {
            print!("{}", s[i]);
        }
    }
}

