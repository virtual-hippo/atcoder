// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        s: usize,
    }
    let mut dp = vec![0; s+1];
    dp[0] = 1;
    for i in 3..s+1 {
        dp[i] = (dp[i-1] + dp[i-3]) % 1_000_000_007;
    }
    println!("{}", dp[s]);
}

