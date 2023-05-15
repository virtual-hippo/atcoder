// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (s,t): (Chars, Chars)
    }
    for i in 0..s.len() {
        if s[i] == t[0] {
            let mut ret = true;
            for j in 1..t.len() {
                if i+j >= s.len() {
                    println!("No");
                    return;
                }
                if s[i+j] == t[j] {
                    //
                } else {
                    ret = false;
                }
            }
            if ret {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

