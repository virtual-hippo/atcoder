// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut with_index: Vec<(usize,usize)> = a.iter().enumerate().map(|(i,&x)|(i,x)).collect();
    with_index.sort_by(|(_,a), (_,b)| a.cmp(b));
    for i in 0..n {
        print!("{} ", with_index[i].0 + 1);
    }
}

