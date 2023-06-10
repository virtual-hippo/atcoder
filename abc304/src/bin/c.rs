// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn is_contain(a1: i64,a2: i64,b1: i64,b2: i64,d: i64) -> bool {
    let d2 = (a1 - b1) * (a1 - b1) + (a2 - b2) * (a2 - b2);
    d2 <= (d * d)
}

fn f(state: &mut Vec<(bool, bool)>, xy: &Vec<(i64,i64)>,d: i64, i:usize) {
    state[i].1 = true;
    for j in 0..state.len() {
        if  state[j].1 {
            continue;
        }
        if is_contain(xy[i].0,xy[i].1,xy[j].0,xy[j].1,d) {
            state[j].0 = true;
            f(state, xy,d, j);
        }
    }
}

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64,i64); n],
    }
    let mut state = vec![(false, false); n];
    state[0].0 = true;
    for i in 0..n {
        if state[i].0 {
            f(&mut state, &xy,d, i);
            // for j in 0..n {
            //     if is_contain(xy[i].0,xy[i].1,xy[j].0,xy[j].1,d) {
            //         state[j].0 = true;
            //     }
            // }
        }
    }
    for i in 0..n {
        if state[i].0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

