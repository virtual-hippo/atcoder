// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::from(["ABC", "ARC", "AGC", "AHC"]);
    input! {
        s: [String; 3],
    }
    for i in 0..3 {
        set.remove(&(s[i].as_str()));
    }
    for v in set.iter() {
        println!("{}", v);
    }
}
