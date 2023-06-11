// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n1: usize,
        n2: usize,
        n3: usize,
        n4: usize,
    }
    if n2 <= n3 {
        println!("0");
    } else if n1 <= n3 && n2 <= n4 {
        println!("{}", n2 - n3);
    }  else if n3 < n1 && n2 <= n4 {
        println!("{}", n2 - n1);
    } else if n3 < n1 && n1 <= n4 && n4 < n2 {
        println!("{}", n4 - n1);
    } else if n4 <= n1 {
        println!("{}", 0);
    } else if n1 <= n3 && n4 <= n2 {
        println!("{}", n4-n3);
    }
}

