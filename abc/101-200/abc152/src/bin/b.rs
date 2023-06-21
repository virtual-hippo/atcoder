// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    }
    let l = (0..b).map(|_| (a + 48) as char).collect::<String>();
    let r = (0..a).map(|_| (b + 48) as char).collect::<String>();
    if l < r {
        println!("{}", l);
    } else {
        println!("{}", r);
    }
}

