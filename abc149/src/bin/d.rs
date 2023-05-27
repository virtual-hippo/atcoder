// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (n,k): (usize, usize),
        (r,s, p): (usize, usize, usize),
        t: Chars,
    }
    let mut pre = (0,0,0);
    let mut ans = 0;
    {
        let mut current = 0;
        for i in 0..n {
            if t[i] == 'r' && i - pre.0 != k {
                pre.0 = i;
                current += p;
            }
            if t[i] == 's' && i - pre.1 != k {
                pre.1 = i;
                current += r;
            }
            if t[i] == 'p' && i - pre.2 != k {
                pre.2 = i;
                current += s;
            }
        }
        ans = std::cmp::max(ans, current);
    }
    
    println!("{}", ans);
}

