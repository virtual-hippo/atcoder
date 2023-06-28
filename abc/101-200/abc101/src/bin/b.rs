// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let n: i32 = s.parse().unwrap();
    let sn = s.chars().map(|ch| ch as i32 - 48).sum::<i32>();
    if n % sn == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

