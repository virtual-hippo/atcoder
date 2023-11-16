// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut current = n;
    loop {
        let x = current / 100;
        let y = (current % 100) / 10;
        let z = current % 10;
        if x * y == z {
            println!("{}", current);
            return;
        }
        current += 1;
    }
}
