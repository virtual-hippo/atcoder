// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        r: u64,
        a: [u64; n],
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut set = HashSet::with_capacity(n + 1);
    for i in 0..n + 1 {
        set.insert(s[i]);
    }
    for i in 0..n + 1 {
        if set.contains(&(s[i] + p))
            && set.contains(&(s[i] + p + q))
            && set.contains(&(s[i] + p + q + r))
        {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
