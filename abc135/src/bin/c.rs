// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n+1],
        mut b: [u64; n],
    }
    let mut ans = 0;
    for i in 0..n {
        if a[i] <= b[i] {
            ans += a[i];
            b[i] -= a[i];
            a[i] = 0;
            if a[i+1] <= b[i] {
                ans += a[i+1];
                a[i+1] = 0;
            } else {
                ans += b[i];
                a[i+1] -= b[i];
                b[i] = 0;
            }
        } else {
            ans += b[i];
            a[i] -= b[i];
            b[i] = 0;
        }
    }
    println!("{}", ans);
}

