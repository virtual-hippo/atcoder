// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let mut map = HashMap::new();
    for i in 0..s.len() {
        *map.entry(s[i]).or_insert(0) += 1;
    }
    let mut ans = 1;
    for i in 1..=s.len() {
        ans *= i;
    }
    for (_,v) in map.iter() {
        let mut current = 1;
        for i in 1..=*v {
            current *= i;
        }
        ans /= current;
    }
    println!("{}", ans);
}

