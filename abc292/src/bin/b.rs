// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut player = vec![0; n+1];
    for _ in 0..q {
        input! {
            query: usize,
            x: usize,
        }
        if query == 1 {
            player[x] += 1;
        } else if query == 2 {
            player[x] += 2;
        } else if query == 3 {
            if player[x] >= 2 {
                println!("Yes");
            } else {
                println!("No");
            }
        } 
    }
}

