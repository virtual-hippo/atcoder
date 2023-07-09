// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: i64,
    }
    let mut diff = HashMap::with_capacity(n+10);
    for _ in 0..n {
        input! {
            a: usize,
            b: i64,
        }
        *diff.entry(1).or_insert(0) += b;
        *diff.entry(a+1).or_insert(0) -= b;
    }
    let mut keys: Vec<usize> = diff.keys().map(|&k| k).collect();
    keys.sort();
    let mut s = vec![0; keys.len()+1];
    for i in 0..keys.len() {
        let cur = diff.get(&keys[i]).unwrap();
        s[i+1] = s[i] + cur;
        if s[i+1] <= k {
            println!("{}", keys[i]);
            return;
        }
    }
}

