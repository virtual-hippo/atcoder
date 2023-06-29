// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        a: [usize; 10],
    }
    let p1 = a[0];
    let p2 = a[a[p1]];
    
    println!("{}", p2);
}

