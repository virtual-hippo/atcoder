// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let mut b = a.clone();
    for i in 1..n {
        b[0][i] = a[0][i-1];
        b[i][n-1] = a[i-1][n-1];
    }
    for i in (0..n-1).rev() {
        b[n-1][i] = a[n-1][i+1];
        b[i][0] = a[i+1][0];
    }
    for i in 0..n {
        for j in 0..n {
            print!("{}", b[i][j]);
        }
        println!("");
    }
    
}

