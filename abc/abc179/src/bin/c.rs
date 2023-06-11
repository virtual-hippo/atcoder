// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    for a in 1..n {
        for b in a..(n/a + 1) {
            if a * b < n {
                if a == b {
                    ans += 1;
                } else {
                    ans += 2;
                }
            }
        }
    }
    println!("{}", ans);
}

