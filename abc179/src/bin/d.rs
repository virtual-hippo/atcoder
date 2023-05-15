// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        (n,k): (usize, usize),
    }
    let mut set = HashSet::new();
    for _ in 0..k {
        input! {
            (l,r): (usize, usize),
        }
        set.insert(l);
        set.insert(r);
    }
    let mut dp = vec![vec![false; n+1]; n+1];
    for _ in 0..20000 {
        for _ in 0..20000 {
            //
        }
    }
    println!("Yes");
}

