// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
    }
    let apples = (0..n).map(|i| l + i as i64).collect::<Vec<i64>>();
    let sum = apples.iter().sum::<i64>();
    let mut abs_apples_sorted = apples.clone();
    abs_apples_sorted.sort_by(|a,b| a.abs().cmp(&b.abs()));
    println!("{}", sum - abs_apples_sorted[0]);
}

