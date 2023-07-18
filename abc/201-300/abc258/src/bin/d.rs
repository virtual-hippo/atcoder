// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }
    let mut ans = std::usize::MAX;
    let mut sum = 0;
    for i in 0..n {
        if i + 1 > x {
            break;
        }
        sum += ab[i].0 + ab[i].1;
        ans = ans.min(sum + ab[i].1 * (x-i-1));
    }

    println!("{}", ans);
}

