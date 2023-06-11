// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    let ans = map.values().fold(0, |sum, x| sum + x/2);
    println!("{}", ans);
}

