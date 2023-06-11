// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (n,x): (usize, usize),
        a: [usize; n],
    }
    let mut table = vec![false; n];
    let mut ans = 0;
    let mut next = x;
    loop {
        if table[next-1] {
            break;
        } else {
            table[next-1] = true;
            next = a[next-1];
            ans += 1;
        }
    }
    println!("{}", ans);
}

