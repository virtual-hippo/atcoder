// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n+1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        graph[a].push(b);
        graph[b].push(a);
    }
    for i in 0..n {
        graph[i+1].sort();
        print!("{} ", graph[i+1].len());
        println!("{}", graph[i+1].iter().map(|i| format!("{}", i)).collect::<Vec<String>>().join(" "));
    }
}

