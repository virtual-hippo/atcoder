// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn calc_d(pos1: &Vec<i64>, pos2: &Vec<i64>) -> i64 {
    (0..pos1.len()).fold(0, |sum, x| sum + (pos2[x]-pos1[x]).pow(2))
}

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[i64; d]; n],
    }
    let mut ans = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            let d2 = calc_d(&x[i], &x[j]);
            let sqrt = (d2 as f64).sqrt().round() as usize;
            if sqrt * sqrt == (d2 as usize) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

