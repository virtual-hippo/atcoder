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
    let ans = (0..n).fold(vec![], |mut vec, i| {
        let mut clone = vec.clone();
        clone.push(i + 1);
        clone.append(&mut vec);
        clone
    });
    ans.iter().for_each(|&v| print!("{} ", v))
}
