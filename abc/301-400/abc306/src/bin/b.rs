// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        a: [u128; 64],
    }
    let mut ans = 0;
    for i in 0..64 {
        ans += a[i] * 2_u128.pow(i as u32);
    }
    println!("{}", ans);
}

