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
    for i in 2..n {
        if s[i - 2] == 'A' && s[i - 1] == 'B' && s[i] == 'C' {
            println!("{}", i - 1);
            return;
        }
    }
    println!("{}", -1);
}
