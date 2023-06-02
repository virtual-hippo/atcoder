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
    let mut graph = vec![vec![]; n];
    for _ in 0..n-1 {
        input! {
            a: usize,
            b: usize,
        }
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    for i in 0..n {
        if graph[i].len() == n-1 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

