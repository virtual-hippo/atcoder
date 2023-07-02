// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: [usize; 8],
    }
    for i in 0..8 {
        if n[i] < 100 {
            println!("No");
            return;
        }
        if 675 < n[i] {
            println!("No");
            return;
        }
        if n[i] % 25 != 0 {
            println!("No");
            return;
        }
    }
    for i in 1..8 {
        if n[i] < n[i-1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

