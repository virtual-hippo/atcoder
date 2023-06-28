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
    let ans = s.iter().map(|&ch| if ch == '+' {1} else {-1}).fold(0, |sum, x| sum + x);
    println!("{}", ans);
}

