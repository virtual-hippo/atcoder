// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut ans = 0;
    ans = ans.max(a + b);
    ans = ans.max(a + c);
    ans = ans.max(b + c);
    println!("{}", ans);
}
