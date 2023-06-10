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
        a: [Chars; n],
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if (a[i][j] == 'W' && a[j][i] != 'L') ||
                (a[i][j] == 'L' && a[j][i] != 'W') ||
                (a[i][j] == 'D' && a[j][i] != 'D') {
                println!("incorrect");
                return;
            }
        }
    }
    println!("correct");
}

