// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let mut state = vec![(false, false); n];
    state[0] = (true, true);
    for i in 1..n {
        if state[i-1].0 {
            if (a[i] - a[i-1]).abs() <= k {
                state[i].0 = true;
            }
            if (b[i] - a[i-1]).abs() <= k {
                state[i].1 = true;
            }
        }
        if state[i-1].1 {
            if (a[i] - b[i-1]).abs() <= k {
                state[i].0 = true;
            }
            if (b[i] - b[i-1]).abs() <= k {
                state[i].1 = true;
            }
        }
        if state[i].0 == false && state[i].1 == false {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

