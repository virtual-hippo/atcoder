// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        v: usize,
        a: usize,
        b: usize,
        c: usize,
    }
    let mut current = v;
    let array = [a,b,c];
    let mut pos = 0;
    while array[pos] <= current {
        current -= array[pos];
        pos += 1;
        pos %= 3;
    }
    if pos == 0 {
        println!("F");
    } else if pos == 1 {
        println!("M");
    } else {
        println!("T");
    }
}

