// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }
    if s[s.len()-1] == 's' {
        s.push('e');
        s.push('s');
    } else {
        s.push('s');
    }
    println!("{}", s.iter().collect::<String>());
}

