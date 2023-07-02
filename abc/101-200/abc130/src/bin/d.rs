// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let mut s = vec![0; n+1];
    for i in 0..n {
        s[i+1] = s[i] + a[i];
    }
    let s_len = s.len();

    let first = if let Some(i) = s.iter().position(|&x| x >= k) {
        i
    } else {
        println!("{}", 0);
        return;
    };

    let mut i = 1;
    let mut current = first;
    let mut ans = (s_len - current) as u64;
    while i < s_len && current < s_len {
        if s[current] - s[i] >= k {
            ans += (s_len - current) as u64;
            i += 1;
        } else {
            current += 1;
        }
    }

    
    println!("{}", ans);
}

