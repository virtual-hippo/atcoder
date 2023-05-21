// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;


fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut state = (0,0);
    for i in 0..n {
        if s[i] == 'T' {
            state.0 += 1;
        } else {
            state.1 += 1;
        }
    }
    if state.0 > state.1 {
        println!("T");
    } else if state.0 < state.1 {
        println!("A");
    } else {
        if s[n-1] == 'T' {
            println!("A");
        } else {
            println!("T");
        }
    }
}

