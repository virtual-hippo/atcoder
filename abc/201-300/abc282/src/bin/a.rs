// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        k: u8,
    }
    let large_a = 65_u8;
    (0..k).for_each(|i| print!("{}", (large_a + i) as char));
}

