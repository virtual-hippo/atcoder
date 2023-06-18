// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    print!("{}", s[0]);
    for i in 1..n {
        if s[i] == 'a' && s[i-1] == 'n' {
            print!("y{}", s[i]);
        } else {
            print!("{}", s[i]);
        }
    }
}

