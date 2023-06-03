// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut b = vec![0; n];
    for i in 0..n/2 {
        b[2 * i + 1] = a.pop().unwrap();
    }
    for i in 0..(n/2)+1 {
        b[2 * i] = a.pop().unwrap();
    }
    for i in 0..n/2 {
        let pos = 2 * i + 1;
        if (b[pos-1] < b[pos] && b[pos] > b[pos+1]) == false {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

