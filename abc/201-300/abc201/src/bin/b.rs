// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, usize); n],
    }
    st.sort_by(|(_, a), (_, b)| b.cmp(a));
    println!("{}", st[1].0);
}

