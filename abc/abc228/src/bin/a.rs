// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (s,t,x): (usize, usize, usize),
    }
    let f = |val| -> bool {
        if s < t {
            s <= val && val < t
        } else {
            (s <= val && val < 24) ||
            val < t
        }
    };
    let ans = if f(x) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

