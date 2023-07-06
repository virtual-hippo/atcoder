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
        s: Chars,
    }
    let mut left = vec![];
    let mut diff = vec![0_i64; n+1];
    for i in 0..n {
        if s[i] == '(' {
            left.push(i)
        }
        if s[i] == ')'  {
            if let Some(v) = left.pop() {
                diff[v] += 1;
                diff[i+1] -= 1;
            }
        }
    }
    
    let mut ruiseki = vec![0_i64; n+2];
    for i in 0..n+1 {
        ruiseki[i+1] = ruiseki[i] + diff[i];
    }
    for i in 0..n {
        if ruiseki[i+1] == 0 {
            print!("{}", s[i]);
        }
    }
}

