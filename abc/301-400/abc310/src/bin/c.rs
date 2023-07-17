use proconio::input;
use std::collections::HashSet;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
    }
    let mut bou_set = HashSet::with_capacity(n);
    for _ in 0..n {
        input! {
            mut s: Chars,
        }
        let rev_s = s.iter().rev().map(|&ch| ch).collect::<Vec<char>>();
        if bou_set.contains(&s) == false && bou_set.contains(&rev_s) == false {
            bou_set.insert(s);
        }
    } 
    println!("{}", bou_set.len());
}

