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
        a: [usize; 4 * n -1],
    }
    let mut map = HashMap::new();
    for i in 0..a.len() {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    for (k,v) in map.iter() {
        if *v == 3 {
            println!("{}", *k);
        }
    }
}

