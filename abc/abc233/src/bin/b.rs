// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars,
    }
    let mut ans = vec![];
    for i in 0..l-1 {
        ans.push(s[i]);
    }
    for i in (l-1..r).rev() {
        ans.push(s[i]);
    }
    for i in r..s.len() {
        ans.push(s[i]);
    }
    for i in 0..s.len() {
        print!("{}", ans[i]);
    }
}

