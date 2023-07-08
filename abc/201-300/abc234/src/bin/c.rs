// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        k: u64,
    }
    let k_str = format!("{:>b}", k);
    let ans: String = k_str.chars().map(|ch| if ch == '1' {'2'} else {'0'}).collect();
    println!("{}", ans);
}

