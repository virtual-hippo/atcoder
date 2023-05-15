// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (a,b): (i32, i32),
    }
    let ans = a+ b;
    let ans = std::cmp::max(ans, a-b);
    let ans = std::cmp::max(ans, a*b);
    println!("{}", ans);
}

