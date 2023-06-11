// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut x = n;

    loop {
        let mut i = 1;
        let mut current = vec![];
        while i * i < x {
            if x % i == 0 {
                current.push(i)
            }
            i+=1;
        }
        if current.len() == 1 {
            println!("{}", x);
            return;
        }
        x += 1;
    }
}

