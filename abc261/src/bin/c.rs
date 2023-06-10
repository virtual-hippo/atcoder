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
    }
    let mut map = HashMap::new();

    for _ in 0..n {
        input! {
            s: String,
        }
        *map.entry(s.clone()).or_insert(0) += 1;
        let cnt = map.get(&s).unwrap();
        if *cnt == 1 {
            println!("{}", s);
        } else {
            println!("{}({})", s, *cnt-1);
        }
    }
}

