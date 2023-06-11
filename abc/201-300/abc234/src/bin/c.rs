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
    // 2,20,22,200,202,220,222,2000,2002,2020,2022,2200,2202,2220,2222
    let mut i = 0;
    let mut current = 0_u64;
    while current < k {
        current += 2_u64.pow(i);
        i += 1;
    }
    println!("{}", (current+1)/2);
    println!("{}", 2 * 10_u64.pow(i-1));
}

