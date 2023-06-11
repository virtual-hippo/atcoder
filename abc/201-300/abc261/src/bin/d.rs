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
        x: [i64; n],
    }
    let mut y = vec![0; n+1];
    for _ in 0..m {
        input! {
            c: usize,
            yy: i64,
        }
        y[c] = yy;
    }
    let mut dp = vec![vec![-1; n+1]; n+2];
    dp[1][0] = 0;
    dp[1][1] = x[0] + y[1];
    dp[2][0] = x[0] + y[1];
    for i in 2..n+1 {
        for j in 1..n+1 {
            if dp[i-1][j-1] == -1 {
                continue;
            }
            dp[i][j] = dp[i-1][j-1] + x[i-1] + y[j];
            dp[i+1][0] = std::cmp::max(dp[i+1][0], dp[i][j]);
        }
    }
    let ans = dp[n].iter().fold(0, |ret, x|std::cmp::max(ret, *x));
    println!("{}", ans);
}

