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
        a: [usize; n],
        b: [usize; m],
    }
    let mut c = vec![(0,0,0); n+m];
    for i in 0..n {
        c[i] = (1, 0, a[i]);
    }
    for i in 0..m {
        c[n+i] = (2, 0 ,b[i]);
    }
    c.sort_by(|a, b| a.2.cmp(&b.2));
    for i in 0..c.len() {
        c[i].1 = i+1;
    }
    for i in 0..c.len() {
        if c[i].0 == 1 {
            if i == c.len() - 1 {
                print!("{}", c[i].1);
            } else {
                print!("{} ", c[i].1);
            }
        }
    }
    println!("");
    for i in 0..c.len() {
        if c[i].0 == 2 {
            if i == c.len() - 1 {
                print!("{}", c[i].1);
            } else {
                print!("{} ", c[i].1);
            }
        }
    }
}

