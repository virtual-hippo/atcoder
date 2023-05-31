// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: usize,
    }
    if c %2 == 0 {
        let a = std::cmp::max(a, -a);
        let b = std::cmp::max(b, -b);
        if a < b {
            println!("<");
        } else if a > b {
            println!(">");
        } else {
            println!("=");
        }
    } else {
        if 0 <= a {
            if 0 <= b {
                if a < b {
                    println!("<");
                } else if a > b {
                    println!(">");
                } else {
                    println!("=");
                }
            } else {
                println!(">");
            }
            return;
        }
        if a < 0 {
            if 0 <= b {
                println!("<");
            } else {
                if a < b {
                    println!("<");
                } else if a > b {
                    println!(">");
                } else {
                    println!("=");
                }
            } 
            return;
        }
    }
}

