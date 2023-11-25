// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }
    let mut flg1 = true;
    let mut flg2 = true;
    for i in 0..n {
        if s[i] != t[i] {
            flg1 = false;
        }
    }
    for i in 0..n {
        if s[i] != t[m - (n - 1 - i) - 1] {
            flg2 = false;
        }
    }
    if flg1 && flg2 {
        println!("{}", 0);
    } else if flg1 {
        println!("{}", 1);
    } else if flg2 {
        println!("{}", 2);
    } else {
        println!("{}", 3);
    }
}
