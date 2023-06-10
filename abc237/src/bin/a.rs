// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: i128,
    }
    if -2_i128.pow(31) <= n && n < 2_i128.pow(31) {
        println!("Yes");
    }  else {
        println!("No");
    }
    
}

