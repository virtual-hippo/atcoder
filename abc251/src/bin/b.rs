// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        (n,w): (usize, usize),
        a: [usize; n],
    }
    let mut set = HashSet::new();
    for i in 0..n-2 {
        for j in i+1..n-1 {
            for k in j+1..n {
                set.insert(a[i] + a[j] + a[k]);
            }
        }
    }
    for i in 0..n-1 {
        for j in i+1..n {
            set.insert(a[i] + a[j]);
        }
    }
    for i in 0..n {
        set.insert(a[i]);
    }
    let ans = set.iter().fold(0, |sum, x| if *x <= w {sum+1} else {sum});
    println!("{}", ans);
}

