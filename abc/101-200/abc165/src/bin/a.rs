// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    }
    let ans = (a..b+1).filter(|&i| i % k == 0).count() > 0;
    if ans {
        println!("OK");
    } else {
        println!("NG");
    }
}

