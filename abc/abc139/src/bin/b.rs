// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (a,b): (usize, usize),
    }
    let mut current = 1;
    let mut ans = 0;
    while current < b {
        current -= 1;
        current += a;
        ans +=1;
    }
    println!("{}", ans);
}

