// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for i in 0..n {
        if a[i] >= 10 {
            ans += a[i] - 10;
        }
    }
    println!("{}", ans);
}

