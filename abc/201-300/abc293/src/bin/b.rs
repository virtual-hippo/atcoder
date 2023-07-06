// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut state = vec![false; n];
    for i in 0..n {
        if state[i] == false {
            state[a[i]-1] = true;
        }
    }
    println!("{}", state.iter().filter(|&&v| v==false).count());
    println!("{}", state.iter().enumerate().filter(|(_, &v)| v==false).map(|(i, _)| format!("{}", i+1)).collect::<Vec<String>>().join(" "));
}

