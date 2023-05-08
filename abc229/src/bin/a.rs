// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
    }
    if s1[0] == '.' && s2[1] == '.' {
        println!("No");
    } else if s1[1] == '.' && s2[0] == '.' {
        println!("No");
    } else {
        println!("Yes");
    }
}

