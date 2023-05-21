// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    for i in 0..n-1 {
        print!("{} ", a[i]);
        if a[i] < a[i+1] {
            if a[i+1] - a[i] == 1 {
                continue;
            }
            for j in (a[i] + 1)..a[i+1] {
                print!("{} ", j);
            }
        } else {
            if a[i] - a[i+1] == 1 {
                continue;
            }
            for j in ((a[i+1] + 1)..a[i]).rev() {
                print!("{} ", j);
            }
        }
    }
    println!("{}", a[n-1]);
}

