// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    
    let mut set = HashSet::new();
    for i in 0..4 {
        input! {
            s: String,
        }
        set.insert(s);
    }
    if set.len() == 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}

