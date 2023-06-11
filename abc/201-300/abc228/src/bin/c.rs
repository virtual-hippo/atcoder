// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }
    let mut point = vec![0; n];
    for i in 0..n {
        input! {
            (x, y, z): (usize, usize, usize),
        }
        point[i] = x + y + z;
    }
    let mut sorted_point = point.clone();
    sorted_point.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let k_point = sorted_point[k-1];
    for i in 0..n {
        if point[i] + 300 >= k_point {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

