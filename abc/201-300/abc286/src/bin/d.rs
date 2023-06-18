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
    let mut dp = vec![vec![false; 1_000_000]; n+1];
    dp[0][0] = true;
    for i in 1..n+1 {
        for j in 0..1_000_000 {
            if dp[i-1][j] {
                dp[i][j] = true;
                for k in 1..=ab[i-1].1 {
                    dp[i][j + ab[i-1].0*k] = true;
                }
            }
        }
    }
    let mut ans = false;
    for i in 1..n+1 {
        if dp[i][x] {
            ans = true;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

