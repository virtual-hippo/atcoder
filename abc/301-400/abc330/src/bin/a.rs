// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }
    let ans = a.iter().filter(|&v| *v >= l).count();
    println!("{}", ans);
}
