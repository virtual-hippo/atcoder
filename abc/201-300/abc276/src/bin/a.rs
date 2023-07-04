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
    if let Some(v) = s.iter().rposition(|&x| x == 'a') {
        println!("{}", v+1);
    } else {
        println!("{}", -1);
    }
}

