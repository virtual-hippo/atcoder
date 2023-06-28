// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        mut a: [usize; 3],
    }
    a.sort();
    if a[2] - a[1] == a[1] - a[0] {
        println!("Yes");
    } else {
        println!("No");
    }
}

