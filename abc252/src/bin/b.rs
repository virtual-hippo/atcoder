// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (n,k): (usize, usize),
        a: [usize; n],
        b: [usize; k],
    }
    let max = a.iter().max().unwrap();
    for i in 0..n {
        if a[i] == *max {
            for j in 0..k {
                if i+1 == b[j] {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}

