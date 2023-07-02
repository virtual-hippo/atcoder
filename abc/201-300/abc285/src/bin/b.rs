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
    for i in 0..n-1 {
        let mut current = 0;
        let l = n-i-1;
        for k in 0..l {
            if s[k] == s[k+i+1] {
                break;
            }
            current = k+1;
        }
        println!("{}", current);
    }
}

