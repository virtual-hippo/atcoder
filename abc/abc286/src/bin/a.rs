// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        a: [usize; n],
    }
    for i in 0..n {
        if p<= i+1 && i+1 <= q {
            print!("{} ", a[i+(r-p)]);
        } else if r<= i+1 && i+1 <= s  {
            print!("{} ", a[i-(r-p)]);
        } else {
            print!("{} ", a[i]);
        }
    }
}

