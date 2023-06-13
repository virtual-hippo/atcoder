// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [i64; n],
    }
    let mut ans = (0..n).fold(0, |sum, x| sum + w[x]); 
    for t in 1..n {
        let s1 = (0..t).fold(0, |sum, x| sum + w[x]);
        let s2 = (t..n).fold(0, |sum, x| sum + w[x]);
        ans = std::cmp::min(ans, (s2-s1).abs());
    }
    println!("{}", ans);
}

