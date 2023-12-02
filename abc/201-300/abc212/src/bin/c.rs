// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    let mut vec = Vec::with_capacity(n + m);
    for i in 0..n {
        vec.push((0, a[i]));
    }
    for i in 0..m {
        vec.push((1, b[i]));
    }
    vec.sort_by(|(_, a), (_, b)| a.cmp(b));

    let mut ans = usize::MAX;
    for i in 1..n + m {
        if vec[i].0 != vec[i - 1].0 {
            ans = ans.min(vec[i].1 - vec[i - 1].1);
        }
    }
    println!("{}", ans);
}
