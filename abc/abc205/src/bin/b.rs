// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            a: usize,
        }
        set.insert(a);
    }
    if set.len() == n {
        println!("Yes");//
    } else {
        println!("No");
    }
    
}

