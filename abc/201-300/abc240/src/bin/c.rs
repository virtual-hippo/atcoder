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
        ab: [(usize,usize); n],
    }
    let mut dp = vec![vec![false; x+1]; n+1];
    dp[0][0] = true;
    for i in 1..n+1 {
        for j in 0..x+1 {
            if ab[i-1].0 <= j && dp[i-1][j - ab[i-1].0] {
                dp[i][j] = true;
            }
            if ab[i-1].1 <= j && dp[i-1][j - ab[i-1].1] {
                dp[i][j] = true;
            }
        }
    }
    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}

