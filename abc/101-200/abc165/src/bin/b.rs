// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        x: u128,
    }
    let mut yokin = 100;
    let mut year = 0;
    while yokin < x {
        yokin = (yokin * 101) / 100;
        year += 1;
    }
    println!("{}",  year);
}

