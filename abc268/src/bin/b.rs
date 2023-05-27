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
    if s.len() > t.len() {
        println!("No");
        return;
    }
    for i in 0..s.len() {
        if s[i] != t[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

