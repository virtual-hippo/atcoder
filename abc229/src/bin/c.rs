// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        (n,w): (usize, usize),
        mut cheese: [(usize, usize); n],
    }
    cheese.sort_by(|a, b| (a.0).cmp(&(b.0)));
    let mut limit = w;
    let mut ans = 0;
    while let Some(current) = cheese.pop() {
        if limit == 0 {
            break;
        } else if current.1 <= limit {
            ans += current.0 * current.1;
            limit -= current.1;
        } else {
            ans += current.0 * limit;
            limit = 0;
        }
    }
    println!("{}", ans);
}

