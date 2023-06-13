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
        a: [usize; m],
    }
    let mut is_broken = vec![false; n+1];
    for i in 0..m {
        is_broken[a[i]] = true;
    }
    let mut dp = vec![0; n+1];
    dp[0] = 1;
    if is_broken[1] == false {
        dp[1] = 1;
    }
    for i in 2..n+1 {
        if is_broken[i-2] == false {
            dp[i] += dp[i-2];
        }
        if is_broken[i-1] == false {
            dp[i] += dp[i-1];
        }
        dp[i] %= 1_000_000_007;
    }
    println!("{}", dp[n]);
}

