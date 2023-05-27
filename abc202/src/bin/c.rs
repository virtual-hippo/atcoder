// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let mut a_ = vec![0_u64; n+1];
    for i in 0..n {
        a_[a[i]] += 1;
    }
    let mut d = vec![0_u64; n+1];
    for i in 0..n {
        d[b[c[i]-1]] += 1;
    }
    let mut ans = 0_u64;
    for i in 0..n+1 {
        ans += a_[i] * d[i];
    }
    println!("{}", ans);
}

