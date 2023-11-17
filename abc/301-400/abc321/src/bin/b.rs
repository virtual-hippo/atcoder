// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        mut a: [i64; n-1],
    }
    let min = a.iter().min().unwrap();
    let max = a.iter().max().unwrap();
    let sum: i64 = a.iter().sum();
    let current = sum - min - max;

    let ans: i64 = if current >= x {
        0
    } else if x - current <= *min {
        0
    } else if x - current <= *max {
        x - current
    } else if x - current > *max {
        if current + max >= x {
            *max
        } else {
            -1
        }
    } else {
        -1
    };

    println!("{}", ans);
}

