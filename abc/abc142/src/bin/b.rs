// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (n,k): (usize, usize),
        h: [usize; n],
    }
    let ans = (0..n).fold(0, |sum, x| if h[x] >= k {sum+1} else {sum});
    println!("{}", ans);
}

