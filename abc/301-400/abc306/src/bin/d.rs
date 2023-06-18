// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, i64); n],
    }
    let mut dp = vec![(0, 0); n+1];
    for i in 1..n+1 {
        let current = xy[i-1];
        if current.0 == 0 {
            // 解毒剤入り
            dp[i].0 = std::cmp::max(
                dp[i-1].0, 
                std::cmp::max(dp[i-1].0 + current.1, dp[i-1].1 + current.1)
            );
            dp[i].1 = dp[i-1].1;
        }  else {
            // 毒入り
            dp[i].0 = dp[i-1].0;

            dp[i].1 = std::cmp::max(
                dp[i-1].1,
                dp[i-1].0 + current.1
            )
        }
    }
    let ans = std::cmp::max(dp[n].0, dp[n].1);
    println!("{}", ans);
}

