// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        p: char,
        q: char,
    }
    let mut map = HashMap::new();
    map.insert('A',0);
    map.insert('B',3);
    map.insert('C',4);
    map.insert('D',8);
    map.insert('E',9);
    map.insert('F',14);
    map.insert('G',23);
    let a = map.get(&p).unwrap();
    let b = map.get(&q).unwrap();
    println!("{}", std::cmp::max(a-b, b-a));
}

