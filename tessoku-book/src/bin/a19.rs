// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (n,w): (usize, usize),
        wv: [(usize, i64); n],
    }

    let mut dp = vec![vec![-1; w+1]; n+1];
    dp[0][0] = 0;
    for i in 1..n+1 {
        for j in 0..(w+1) {
            if dp[i-1][j] != -1 {
                dp[i][j] = std::cmp::max(dp[i-1][j], dp[i][j]);
                if j+wv[i-1].0 < w+1  {
                    dp[i][j+wv[i-1].0] = dp[i-1][j] + wv[i-1].1;
                    dp[i][j+wv[i-1].0] = std::cmp::max(dp[i][j+wv[i-1].0], dp[i-1][j] + wv[i-1].1);
                }
            }
        }
    }
    let ans = dp[n].iter().fold(0, |max, val| std::cmp::max(max, *val));
    println!("{}", ans);
}

