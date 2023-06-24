// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut ans = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            let mut current = true;
            for k in 0..m {
                if s[i][k] == 'x' && s[j][k] == 'x' {
                    current = false;
                }
            }
            if current {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

