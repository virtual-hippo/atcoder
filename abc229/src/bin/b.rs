// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (a,b): (usize, usize),
    }
    let mut ans = false;
    for i in 0..19 {
        let current = (a / (10_i64.pow(i) as usize), b / (10_i64.pow(i) as usize));
        if current.0 % 10 + current.1 % 10 >= 10 {
            ans = true;
            break;
        }
    }
    if ans {
        println!("Hard");
    } else {
        println!("Easy");
    }
}

