// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }
    let mut x = vec![];
    for i in 0..n {
        if l <= a[i] && a[i] <= r {
            x.push(a[i]);
        } else if a[i] < l {
            x.push(l);
        } else {
            x.push(r);
        }
    }

    print!("{}", x[0]);
    for i in 1..n {
        print!(" {}", x[i]);
    }
}
