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
    let mut kakuritu = vec![(0, (0,0)); n];
    for i in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        kakuritu[i] = (i, (a,b));
    }
    kakuritu.sort_by(|(_,(a1,b1)),(_,(a2,b2))| ((a1+b1)*a2).cmp(&((a2+b2)*a1)));
    for i in 0..n {
        print!("{} ", kakuritu[i].0 + 1);
    }
}

