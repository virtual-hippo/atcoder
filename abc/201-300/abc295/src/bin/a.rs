// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }
    for i in 0..n {
        if w[i] == "and" {
            println!("Yes");
            return;
        } else if w[i] == "not" {
            println!("Yes");
            return;
        } else if w[i] == "that" {
            println!("Yes");
            return;
        } else if w[i] == "the" {
            println!("Yes");
            return;
        } else if w[i] == "you" {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

