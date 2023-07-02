// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let ans = s
        .iter()
        .enumerate()
        .map(|(i, &ch)| (ch as u64 - 64) * 26_u64.pow((s.len()-1-i) as u32))
        .sum::<u64>();
    println!("{}", ans);
}
