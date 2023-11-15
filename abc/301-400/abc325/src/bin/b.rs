// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut vec = vec![0; 100];
    for _ in 0..n {
        input! {
            w: i64,
            x: usize,
        }
        let l = 9 + x;
        let r = 18 + x;
        vec[l] += w;
        vec[r] -= w;

        let l = 24 + 9 + x;
        let r = 24 + 18 + x;
        vec[l] += w;
        vec[r] -= w;
    }
    let mut s = vec![0; 101];
    for i in 0..100 {
        s[i + 1] = s[i] + vec[i]
    }
    let ans = s.iter().max().unwrap();
    println!("{}", ans);
}
