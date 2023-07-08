// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use std::collections::BinaryHeap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [i64; n],
    }
    let mut b_heap = BinaryHeap::with_capacity(k+1);
    for i in 0..k {
        b_heap.push(-p[i]);
    }
    println!("{}", -1 * *(b_heap.peek().unwrap()));

    for i in k..n {
        b_heap.push(-p[i]);
        b_heap.pop();
        println!("{}", -1 * *(b_heap.peek().unwrap()));
    }
}
