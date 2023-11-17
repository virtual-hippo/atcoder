// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }

    if n.len() == 1 {
        println!("{}", "Yes");
        return;
    }
    for i in 1..n.len() {
        if n[i - 1] <= n[i] {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
